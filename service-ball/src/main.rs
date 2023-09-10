use warp::Filter;
use warp::http::StatusCode;
use warp::reply::json;


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
    let ball_states = vec![0,0,0,0,1,1,1,1]; // Just a few balls to start
    
    Ok(warp::reply::with_status(json(&ball_states),  StatusCode::OK))
}