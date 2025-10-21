use axum::{ Router };
mod router;
mod middleware;

#[tokio::main]
async fn main() {

    let app: Router = router::create_routers();

    let address: String = format!("0.0.0.0:{}", 8080);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server is running on {address}");
    axum::serve(listener, app).await.unwrap();
}
