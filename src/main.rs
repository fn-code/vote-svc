use actix_web::{HttpResponse, Responder, web};
use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::{oneshot, Mutex};
use vote_svc::app;
use vote_svc::candidate;
use vote_svc::infrastructure::database::postgres;
use vote_svc::infrastructure::http::server;
use vote_svc::infrastructure::config;
use dotenvy;



#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok(); // load .env into process env

    match config::AppConfig::init() {
        Ok(_) => println!("Configuration loaded successfully."),
        Err(e) => {
            panic!("Failed to load configuration: {}", e);
        }
    };

    // use it locally in main
    let cfg =config::AppConfig::global();

    
    let config = server::Config {
        address: cfg.server.addr.clone(),
        port: cfg.server.port,
    };
    let mut server = server::Server::new(config);

    let postgres_config = postgres::PostgresConfig {
        address: cfg.database.addr.clone(),
        port: cfg.database.port,
        dbname: cfg.database.name.clone(),
        username: cfg.database.username.clone(),
        password: cfg.database.password.clone(),
        max_conn: cfg.database.max_conn,
        min_conn: cfg.database.min_conn,
    };

    let mut postgres_conn = postgres::Postgres::new(postgres_config);

    println!("Trying to run database instance...");
    match postgres_conn.connect().await {
        Ok(_) => println!("Successfully connected to database..."),
        Err(e) => panic!("Database connection failed to establish: {}", e),
    }

    let postgres_arc = Arc::new(Mutex::new(postgres_conn));
    let mut postgres_for_shutdown = Arc::clone(&postgres_arc);

    //candidate repo
    let candidate_repo = match candidate::repository::PostgresRepo::new(postgres_arc).await {
        Ok(repo) => repo,
        Err(e) => panic!("Database connection failed to establish: {}", e),
    };

    let candidate_repo_arc = Arc::new(candidate_repo);

    // candidate usecase
    let candidate_uc = candidate::usecase::UseCase::new(candidate_repo_arc);

    let app_data = app::AppHandlerData { candidate_uc };

    server.add_routers(candidate::delivery::http::routes);
    let mut server = server; // keep `server` as owned value

    // Create a oneshot channel to signal shutdown
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();


    let server_task = tokio::spawn(async move {
        tokio::select! {
            res = async {
                server.start(app_data).await
            } => {
                res
            }
            _ = shutdown_rx => {
                println!("Shutdown signal received inside server task, stopping server...");
                server.stop().await;
                Ok(())
            }
        }
    });

    // Spawn ctrl-c handler that sends the shutdown signal
    {
        let shutdown_tx = shutdown_tx;
        tokio::spawn(async move {
            if signal::ctrl_c().await.is_ok() {
                println!("\nCtrl+C received.");
                // Tell the server task to stop
                let _ = shutdown_tx.send(());
                println!("Shutdown signal sent to server task.");
            }
        });
    }



    // Wait
    match server_task.await {
        Ok(Ok(())) => println!("Server task exited gracefully."),
        Ok(Err(e)) => eprintln!("Server task returned an error: {:?}", e),
        Err(join_err) => eprintln!("Server task panicked or was cancelled: {:?}", join_err),
    }

    let mut psql_mutex = postgres_for_shutdown.lock().await;
    println!("Trying closing database...");

    psql_mutex.close().await;
    println!("Successfully closing database...");

    Ok(())
}
