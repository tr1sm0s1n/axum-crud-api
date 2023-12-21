pub mod routes {
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
        Json,
    };

    use crate::{models::Certificate, types::Db};

    pub async fn create_one(
        State(db): State<Db>,
        Json(input): Json<Certificate>,
    ) -> impl IntoResponse {
        println!("{:#?}", input);
        db.write().unwrap().insert(input.id, input.clone());
        (StatusCode::CREATED, Json(input))
    }

    pub async fn read_all(State(db): State<Db>) -> impl IntoResponse {
        let certificates = db.read().unwrap();
        println!("{:#?}", certificates);
        (StatusCode::OK, Json(certificates.clone()))
    }

    pub async fn read_one(Path(id): Path<u32>, State(db): State<Db>) -> impl IntoResponse {
        let certificate = db.read().unwrap().get(&id).cloned();
        println!("{:#?}", certificate);
        if certificate.is_none() {
            return (StatusCode::NOT_FOUND, Json(certificate));
        }
        (StatusCode::OK, Json(certificate))
    }

    pub async fn update_one(
        Path(id): Path<u32>,
        State(db): State<Db>,
        Json(input): Json<Certificate>,
    ) -> impl IntoResponse {
        println!("{}", "update");
        let certificate = db.read().unwrap().get(&id).cloned();
        if certificate.is_none() {
            return (StatusCode::NOT_FOUND, Json(certificate));
        }
        println!("{:#?}", input);
        db.write().unwrap().insert(id, input.clone());
        (StatusCode::OK, Json(Some(input)))
    }

    pub async fn delete_one(Path(id): Path<u32>, State(db): State<Db>) -> impl IntoResponse {
        let certificate = db.write().unwrap().remove(&id);
        println!("{:#?}", certificate);
        if certificate.is_some() {
            (StatusCode::OK, Json(certificate))
        } else {
            (StatusCode::NOT_FOUND, Json(certificate))
        }
    }
}
