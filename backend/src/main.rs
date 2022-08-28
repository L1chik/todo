// #![allow(unused)]


mod model;
use model::database::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
	// CHECK
	let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;

    dbg!(result.len());
    Ok(())
}
