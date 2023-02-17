use rest_actix_web::run;
use std::{env, io::Result, net::TcpListener};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let address = format!(
        "{}:{}",
        env::var("APP_HOST").unwrap(),
        env::var("APP_PORT").unwrap()
    );
    println!("server running on {}", address);

    match TcpListener::bind(&address) {
        Ok(listener) => {
            run(listener)?.await?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
