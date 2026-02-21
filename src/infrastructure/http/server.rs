use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;


type AppServiceConfig = Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync + 'static>;

pub struct Config {
    pub address: String,
    pub port: u16,
}


pub struct Server {
    config: Config,
    server_handle: Option<Arc<Mutex<actix_web::dev::ServerHandle>>>,
    routes: Vec<AppServiceConfig>,


}

impl Server {
    pub fn new(cfg: Config) -> Self {
       Server {
           config: cfg,
           server_handle: None,
           routes: Vec::new(),
       }
    }


    pub fn add_routers<F>(&mut self, route: F)
    where F: Fn(&mut web::ServiceConfig) + Send + Sync + 'static
    {
        self.routes.push(Arc::new(route));
    }


    pub async fn start<T: Send + Sync + 'static>(&mut self, app_data: T) -> std::io::Result<()> {

        let bind_addr = format!("{}:{}", self.config.address, self.config.port);

        let routes = Arc::new(self.routes.clone());
        let web_data = web::Data::new(app_data);

        let server = HttpServer::new(move || {

            let mut app = App::new().app_data(web_data.clone());

            for route_fn in routes.iter() {
                app = app.configure(|cfg| route_fn(cfg));
            }


            app
        }).bind(bind_addr)?.run();

        let handle = server.handle();
        self.server_handle = Some(Arc::new(Mutex::new(handle)));

        server.await

    }

    pub async fn stop(&self) {
        println!("Server stopped listening on http://{}:{}", self.config.address, self.config.port);

        if let Some(handle) = &self.server_handle {
            println!("Server stopped listening on http://{}", self.config.address);

            let handle = handle.lock().await;
            handle.stop(true).await;
            println!("Server stopped");
        }
    }
}