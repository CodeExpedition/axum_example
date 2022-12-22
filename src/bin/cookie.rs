use axum::{Router, routing::{get}};
use axum::response::{Html, IntoResponse};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/check_visit", get(cookie_handler))
    .layer(CookieManagerLayer::new());



    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn cookie_handler(cookies: Cookies) -> impl IntoResponse {
    let response_html;
    match cookies.get("VISIT") {
        Some(_) => {
            // 쿠키가 있다는 것은 한 번 다녀간 적이 있는 사람
            println!("again visitting user");
            response_html= Html("<html><body> 두 번째 이후</body></html>")
        }
        None => {
            println!("you are the first visitting now!! welcome");
            cookies.add(Cookie::new("VISIT", "TRUE"));
            response_html = Html("<html><body>첫방문</body></html>")
        }
    }
    response_html
}