pub mod mysql;
pub mod postgres;

use notification::port::Database;
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, Error, MySql, Pool, Postgres};

#[derive(sqlx::FromRow)]
pub struct Counter {
    pub count: u16,
}

#[derive(sqlx::FromRow)]
pub struct RowNotification {
    pub title: String,
    pub detail: String,
    pub catgory: String,
    pub created_at: u16,
}

pub enum ConnectionDriver {
    Mysql,
    Postgres,
}

pub struct Connection {
    driver: ConnectionDriver,
    dsn: String,
}

impl Connection {
    pub fn new(driver: ConnectionDriver, dsn: &str) -> Self {
        Self {
            driver,
            dsn: dsn.to_string(),
        }
    }

    pub async fn connect(self) -> Result<Box<dyn Database>, Error> {
        match self.driver {
            ConnectionDriver::Mysql => {
                let pool: Pool<MySql> = MySqlPoolOptions::new().connect(self.dsn.as_str()).await?;
                return Ok(Box::new(mysql::Repository::new(pool)));
            }
            ConnectionDriver::Postgres => {
                let pool: Pool<Postgres> = PgPoolOptions::new().connect(self.dsn.as_str()).await?;
                return Ok(Box::new(postgres::Repository::new(pool)));
            }
        }
    }
}
