mod handlers;
mod models;
mod types;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use handlers::routes;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initializing storage
    let db = types::Db::default();

    // logging middleware
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "test_axum_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with multiple routes
    let app = Router::new()
        .route("/", get(home))
        .route("/create", post(routes::create_one))
        .route("/read", get(routes::read_all))
        .route("/read/:id", get(routes::read_one))
        .route("/update/:id", put(routes::update_one))
        .route("/delete/:id", delete(routes::delete_one))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // println!(">> Listening on {}", listener.local_addr().unwrap());
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Hello, World!"
}
