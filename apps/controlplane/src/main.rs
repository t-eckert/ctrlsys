mod api;

#[tokio::main]
async fn main() {
    let app = api::routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Control Plane listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
