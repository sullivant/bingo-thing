use std::array;
use std::error::Error;

use warp::Filter;
use warp::http::StatusCode;
use warp::reply::json;
use rand::Rng;


const GPIO_STATUS_LED: u8 = 21;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let status_pin = io::status_pin();
    let api = filters::status(status_pin);
    let routes = api
        .with(warp::log("status"))
        .with(warp::cors().allow_any_origin());

    // // GET /health
    // let route_health = warp::path!("health")
    //     .map(|| warp::reply::with_status(format!("Hello from ball state."),  StatusCode::OK));
    
    // // GET /balls
    // let route_ball = warp::path!("balls")
    //     .and_then(|| handle_ball_state());

    // let routes = route_ball
    //     .or(route_health)
    //     .with(warp::cors().allow_any_origin());

    println!("Starting service-ball server.");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;

    Ok(())
}



mod io {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
    use rppal::system::DeviceInfo;

    pub type OutPin = Arc<Mutex<OutputPin>>;
    pub fn status_pin() -> OutPin {
        Arc::new(Mutex::new(Gpio::new().unwrap().get(super::GPIO_STATUS_LED).unwrap().into_output()))
    }
}

mod filters {
    use super::handlers;
    use super::io::OutPin;
    use warp::Filter;
   
    // Status filters
    pub fn status(pin: OutPin) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        status_toggle(pin.clone())
    }

    // POST on /status/toggle will toggle status light
    pub fn status_toggle(pin: OutPin) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let prefix = warp::path!("status" / "toggle");

        prefix.and(warp::post())
        .and(with_pin(pin))
        .and_then(handlers::toggle_status_led)
    }

    fn with_pin(pin: OutPin) -> impl Filter<Extract = (OutPin,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pin.clone())
    }

}

mod handlers {
    use super::io::OutPin;
    use std::convert::Infallible;
    use warp::http::StatusCode;
    use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
    use rppal::system::DeviceInfo;

    pub async fn toggle_status_led(pin: OutPin) -> Result<impl warp::Reply, Infallible> {
        println!("Toggling status led.");  
        let mut status_pin = pin.lock().await;
    
        if status_pin.is_set_high() {
            status_pin.set_low();
        } else {
            status_pin.set_high();
        }
    
       Ok(warp::reply::with_status(format!("Toggled status led state."),  StatusCode::OK))
    }
    

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