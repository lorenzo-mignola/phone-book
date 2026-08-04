use serde_json::Value;

mod util;

#[tokio::test]
async fn should_get_all_contacts() {
    let server = util::setup_test().await;

    let response = server.get("/contacts").await;

    response.assert_status_ok();

    let body: Vec<Value> = response.json();
    assert_eq!(body.len(), 1);
}
