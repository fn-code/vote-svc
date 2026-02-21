use sqlx::{postgres::PgPoolOptions, postgres::PgPool};

#[derive(Debug)]
pub struct PostgresConfig {
    pub address: String,
    pub port: u16,
    pub dbname: String,
    pub username: String,
    pub password: String,
    pub max_conn: u32,
    pub min_conn: u32,
}

pub struct Postgres {
    config: PostgresConfig,
    pool: Option<sqlx::Pool<sqlx::Postgres>>,
}



impl Postgres {

    pub fn new(config: PostgresConfig) -> Postgres {
        Postgres {
            config,
            pool: None
        }
    }

    pub async fn connect(&mut self) -> Result<(), sqlx::Error> {

        let addr = format!("postgres://{}:{}@{}:{}/{}", self.config.username, self.config.password, self.config.address, self.config.port, self.config.dbname).to_string();
        let pool = PgPoolOptions::new()
            .max_connections(self.config.max_conn)
            .min_connections(self.config.min_conn)
            .connect(&addr)
            .await?;

        self.pool = Some(pool);



        Ok(())
    }

    pub fn pool(&self) -> Option<PgPool> {
        self.pool.clone()
    }


    // pool_ref is return as direct reference
    pub fn pool_ref(&self) -> Option<&PgPool> {
        self.pool.as_ref()
    }

    pub async fn close(&mut self) {
        if let Some(pool) = self.pool.take() {
            pool.close().await
        }
    }

    pub async fn close_owned(self) {
        if let Some(pool) = self.pool {
            pool.close().await;
        }
        // `self` drops here
    }
}