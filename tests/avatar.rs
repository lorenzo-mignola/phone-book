mod util;

#[tokio::test]
async fn should_return_avatar() {
    let server = util::setup_test().await;

    let response = server.get("/api/contacts/1/avatar").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn should_not_return_avatar() {
    let server = util::setup_test().await;

    let response = server.get("/api/contacts/100/avatar").await;

    response.assert_status_not_found();
}
