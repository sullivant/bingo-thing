use std::convert::Infallible;

use warp::Filter;

use warp::{http::StatusCode};

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health")
        .map(|| format!("I am healthy!"));
    let routes = health_route
        .with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// pub async fn handle_ball_state() -> Result<impl warp::Reply, Infallible> {
//     let result = "hello".to_string();
    
//     Ok(warp::reply::json(&result))
// }