use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};

use http_body_util::BodyExt; // for `collect`
use serde_json::{json, Value};
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

use crate::app;

#[tokio::test]
async fn home() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}

#[tokio::test]
async fn not_found() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn create_one() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/create")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(
                    &json!({ "id": 56, "name": "Eirene", "course": "MBCC", "status": true, "date": "21-12-2022" })).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({ "id": 56, "name": "Eirene", "course": "MBCC", "status": true, "date": "21-12-2022" })
    );
}

#[tokio::test]
async fn read_all() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/read").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn read_one() {
    let mut app = app().into_service();

    let request = Request::builder()
    .method(http::Method::POST)
    .uri("/create")
    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    .body(Body::from(serde_json::to_vec(
        &json!({ "id": 36, "name": "Raven", "course": "MBCC", "status": true, "date": "24-12-2023" })).unwrap())).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let request = Request::builder()
        .uri("/read/36")
        .body(Body::empty())
        .unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({ "id": 36, "name": "Raven", "course": "MBCC", "status": true, "date": "24-12-2023" })
    );
}

