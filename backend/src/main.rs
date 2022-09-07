// #![allow(unused)]


mod model;
mod security;

use model::database::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
