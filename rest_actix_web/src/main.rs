use rest_actix_web::app;
use std::{env, io::Result, net::TcpListener};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let host: String = env::var("APP_HOST").unwrap_or_default();
    let port: String = env::var("APP_PORT").unwrap_or_default();
    let address: String = format!("{host}:{port}");

    match TcpListener::bind(address) {
        Ok(listener) => {
            println!(
                "server running on port {}",
                listener.local_addr().unwrap().port()
            );

            app::run(listener)?.await?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
