use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let port = env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    warp::serve(hello)
        .run(([127, 0, 0, 1], port))
        .await;
}
