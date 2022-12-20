use axum::Router;
use axum::routing::get;
use axum:: response::Html;
use axum::http::Request;
use axum::body::Body;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn handler(request: Request<Body>) -> Html<&'static str> {
    println!("{:?}", request);
    "<html><body>hello</body></html>".into()
}