use std::array;

use warp::Filter;
use warp::http::StatusCode;
use warp::reply::json;
use rand::Rng;


#[tokio::main]
async fn main() {

    // GET /health
    let route_health = warp::path!("health")
        .map(|| warp::reply::with_status(format!("Hello from ball state."),  StatusCode::OK));
    
    // GET /balls
    let route_ball = warp::path!("balls")
        .and_then(|| handle_ball_state());

    let routes = route_ball
        .or(route_health)
        .with(warp::cors().allow_any_origin());

    println!("Starting service-ball server.");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

/// GET /balls
/// 
/// Will return the current state of all ball contact switches (75 in total).  This is done
/// by querying the GPIO that is connected to the PISO Shift Registers.
/// 
/// TODO: Implement in hardware, for now this just returns 75 static bits that change only slightly
///       to mimic a game in progress.
pub async fn handle_ball_state() -> Result<impl warp::Reply, warp::Rejection> {
    let array_size = 75;
    let mut ball_states = Vec::with_capacity(array_size);
    let mut rng = rand::thread_rng();

    for _ in 0..array_size {
        let this_bit: u8 = rng.gen_range(0..=1);
        ball_states.push(this_bit);
    }
    
    Ok(warp::reply::with_status(json(&ball_states),  StatusCode::OK))
}