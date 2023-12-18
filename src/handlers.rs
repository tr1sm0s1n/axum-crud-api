pub mod routes {
    use axum::{http::StatusCode, response::IntoResponse, Json};

    use crate::models::Certificate;

    pub async fn create(Json(input): Json<Certificate>) -> impl IntoResponse {
        println!("{:?}", input);
        (StatusCode::CREATED, Json(input))
    }

    pub async fn read() -> &'static str {
        "Read all users"
    }

    pub async fn update() -> &'static str {
        "Updated a user"
    }

    pub async fn delete() -> &'static str {
        "Deleted a user"
    }
}
