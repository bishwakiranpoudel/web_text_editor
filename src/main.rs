use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main]
async fn main() {
    let text = Arc::new(Mutex::new(String::new()));

    // Serve the index.html file
    let index_route = warp::get().and(warp::fs::file("src/index.html"));

    // API endpoint for getting and setting text
    let text_api = warp::path("api")
        .and(warp::path("text"))
        .and(warp::filters::body::content_length_limit(1024 * 16))
        .and(warp::body::bytes())
        .and(with_text(text.clone()))
        .and_then(update_text);

    let routes = index_route.or(text_api);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_text(
    text: Arc<Mutex<String>>,
) -> impl Filter<Extract = (Arc<Mutex<String>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || text.clone())
}

async fn update_text(
    body: warp::hyper::body::Bytes,
    text: Arc<Mutex<String>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut text_lock = text.lock().unwrap();
    *text_lock = String::from_utf8_lossy(&body).to_string();
    Ok(warp::reply::html("Text updated successfully"))
}
