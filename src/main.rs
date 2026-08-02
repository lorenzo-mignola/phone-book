mod routes;

#[tokio::main]
async fn main() {
    let app = routes::router();

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("🚀 Server starting at {}", address);
    axum::serve(listener, app)
        .await
        .expect("Error starting server");
}
