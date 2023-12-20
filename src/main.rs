mod handlers;
mod models;
mod types;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use handlers::routes;

#[tokio::main]
async fn main() {
    // initializing storage
    let db = types::Db::default();

    // build our application with multiple routes
    let app = Router::new()
        .route("/", get(home))
        .with_state(db);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!(">> Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Hello, World!"
}
