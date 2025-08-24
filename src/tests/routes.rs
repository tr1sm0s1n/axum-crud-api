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
                    &json!({ "id": 21, "name": "Shin", "course": "MBCC", "status": true, "date": "15-11-2024" })).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({ "id": 21, "name": "Shin", "course": "MBCC", "status": true, "date": "15-11-2024" })
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
        &json!({ "id": 310, "name": "Cassian", "course": "MBCC", "status": true, "date": "19-11-2024" })).unwrap())).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let request = Request::builder()
        .uri("/read/310")
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
        json!({ "id": 310, "name": "Cassian", "course": "MBCC", "status": true, "date": "19-11-2024" })
    );
}

#[tokio::test]
async fn update_one() {
    let mut app = app().into_service();

    let request = Request::builder()
    .method(http::Method::POST)
    .uri("/create")
    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    .body(Body::from(serde_json::to_vec(
        &json!({ "id": 98, "name": "Hella", "course": "MBCC", "status": false, "date": "02-12-2024" })).unwrap())).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let request = Request::builder()
    .method(http::Method::PUT)
    .uri("/update/98")
    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    .body(Body::from(serde_json::to_vec(
        &json!({ "id": 98, "name": "Necresta Hella", "course": "MBCC", "status": true, "date": "02-12-2024" })).unwrap())).unwrap();
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
        json!({ "id": 98, "name": "Necresta Hella", "course": "MBCC", "status": true, "date": "02-12-2024" })
    );
}

#[tokio::test]
async fn delete_one() {
    let mut app = app().into_service();

    let request = Request::builder()
    .method(http::Method::POST)
    .uri("/create")
    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    .body(Body::from(serde_json::to_vec(
        &json!({ "id": 627, "name": "Siglinde", "course": "MBCC", "status": false, "date": null })).unwrap())).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let request = Request::builder()
        .method(http::Method::DELETE)
        .uri("/delete/627")
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
        json!({"id": 627, "name": "Siglinde", "course": "MBCC", "status": false, "date": null})
    );
}
