use axum::{
    routing::get, Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use axum_error::Result;
// use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer

#[tokio::main]
async fn main() -> Result<()>{
    let _= dotenv::dotenv();
    let url= std::env::var("DATABASE_URL")?;
    let pool= SqlitePool::connect(&url).await?;

    let app= Router::new()
        .route("/",get(list))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    
    let address= SocketAddr::from(([0,0,0,0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

async fn list() -> String{
    format!("Hello")
}