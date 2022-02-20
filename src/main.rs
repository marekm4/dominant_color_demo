use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./index.html"));
    let css = warp::path("css").and(warp::fs::dir("./css/"));
    let js = warp::path("js").and(warp::fs::dir("./js/"));
    let routes = index.or(css).or(js);

    let port = env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
