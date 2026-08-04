use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

mod util;

#[tokio::test]
async fn should_get_all_contacts() {
    let app = util::setup_test().await;

    let response = app
        .oneshot(Request::get("/contacts").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
