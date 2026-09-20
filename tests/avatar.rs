use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

mod util;

#[tokio::test]
async fn should_return_avatar() {
    let mock_server = setup_mock().await;
    let server = util::setup_test_with_dicebar(&mock_server.uri()).await;

    let response = server.get("/api/contacts/1/avatar").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn should_not_return_avatar() {
    let server = util::setup_test().await;

    let response = server.get("/api/contacts/100/avatar").await;

    response.assert_status_not_found();
}

async fn setup_mock() -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/10.x/blobs/svg"))
        .and(query_param("seed", "test_"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    mock_server
}
