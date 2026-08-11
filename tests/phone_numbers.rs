use serde_json::Value;

mod util;

#[tokio::test]
async fn should_get_phone_numbers_of_a_contact() {
    let server = util::setup_test().await;

    let response = server.get("/api/contacts/1/phone_numbers").await;

    response.assert_status_ok();

    let body: Vec<Value> = response.json();
    assert_eq!(body.len(), 1);
}
