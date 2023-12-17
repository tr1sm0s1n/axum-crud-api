pub mod handlers;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use handlers::routes;

#[tokio::main]
async fn main() {
    // build our application with multiple routes
    let app = Router::new()
        .route("/", get(home))
        .route("/create", post(routes::create))
        .route("/read", get(routes::read))
        .route("/update", put(routes::update))
        .route("/delete", delete(routes::delete));

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
