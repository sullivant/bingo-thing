use std::array;
use std::error::Error;
use std::process::Output;

use warp::Filter;
use warp::http::StatusCode;

use std::sync::Arc;
use tokio::sync::Mutex;

const GPIO_STATUS_LED: u8 = 21;
const GPIO_SHLD: u8 = 20; // When HIGH data is shifted, when LOW data is loaded from inputs
const GPIO_CLK: u8 = 6; // Clock
const GPIO_SERIAL_DATA: u8 = 19; // Data sent serially from 74HC165N.

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let status_pin = io::status_pin();
    let cloned_pin = status_pin.clone();

    let shld_pin = io::shld_pin();
    let clk_pin = io::clk_pin();
    let data_pin = io::data_pin();

    let api = filters::route_filters(status_pin, shld_pin, clk_pin, data_pin);
    let routes = api
        .with(warp::log("status"))
        .with(warp::cors().allow_any_origin());

    io::set_status(cloned_pin);

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
    pub type InPin = Arc<Mutex<InputPin>>;

    pub fn status_pin() -> OutPin {
        Arc::new(Mutex::new(Gpio::new().unwrap().get(super::GPIO_STATUS_LED).unwrap().into_output()))
    }

    pub fn shld_pin() -> OutPin {
        Arc::new(Mutex::new(Gpio::new().unwrap().get(super::GPIO_SHLD).unwrap().into_output()))
    }

    pub fn clk_pin() -> OutPin {
        Arc::new(Mutex::new(Gpio::new().unwrap().get(super::GPIO_CLK).unwrap().into_output()))
    }

    pub fn data_pin() -> InPin {
        Arc::new(Mutex::new(Gpio::new().unwrap().get(super::GPIO_SERIAL_DATA).unwrap().into_input()))
    }

    pub async fn set_status(status_pin: Arc<Mutex<OutputPin>>) {
        println!("Setting Status Pin.");
        let mut this_pin = status_pin.lock().await;
        this_pin.set_high();
    }

}

mod filters {
    use crate::io::InPin;

    use super::handlers;
    use super::io::OutPin;
    use warp::Filter;
   
    // pin related filters
    pub fn route_filters(status: OutPin, shld: OutPin, clk: OutPin, data: InPin) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        get_ball_states(status.clone())
        .or(status_toggle(status.clone()))
        .or(report_status(status.clone(), shld.clone(), clk.clone(), data.clone()))
    }

    pub fn get_ball_states(pin: OutPin) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let prefix = warp::path!("balls");
        prefix.and(warp::get())
        .and(with_out_pin(pin))
        .and_then(handlers::get_ball_states)
    }

    // GET on /status will log state of important pints
    pub fn report_status(status_pin: OutPin, shld_pin: OutPin, clk_pin: OutPin, data_pin: InPin) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let prefix = warp::path!("status");
        prefix.and(warp::get())
        .and(with_out_pin(status_pin))
        .and(with_out_pin(shld_pin))
        .and(with_out_pin(clk_pin))
        .and(with_in_pin(data_pin))
        .and_then(handlers::report_status)
    }


    // POST on /status/toggle will toggle status light
    pub fn status_toggle(pin: OutPin) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let prefix = warp::path!("status" / "toggle");

        prefix.and(warp::post())
        .and(with_out_pin(pin))
        .and_then(handlers::toggle_status_led)
    }

    fn with_out_pin(pin: OutPin) -> impl Filter<Extract = (OutPin,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pin.clone())
    }

    fn with_in_pin(pin: InPin) -> impl Filter<Extract = (InPin,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pin.clone())
    }

}

mod handlers {
    use crate::io::InPin;

    use super::io::OutPin;
    use std::convert::Infallible;
    use warp::http::StatusCode;
    use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
    use rppal::system::DeviceInfo;
    use warp::reply::json;
    use rand::Rng;

    /// PUT /status/toggle
    /// 
    /// Does the actual toggling of the status led
    pub async fn toggle_status_led(pin: OutPin) -> Result<impl warp::Reply, Infallible> {
        println!("Toggling status led.");  
        let mut status_pin = pin.lock().await;

        status_pin.toggle();
    
        Ok(warp::reply::with_status(format!("Toggled status led state."),  StatusCode::OK))
    }

    /// GET /status
    /// 
    /// Just reports (first, to log/println) the status of all relevant IOs
    pub async fn report_status(status_pin: OutPin, shld_pin: OutPin, clk_pin: OutPin, data_pin: InPin) -> Result<impl warp::Reply, Infallible> {
        println!("Logging status of relevant GPIO pins: ");

        let status = status_pin.lock().await;
        println!("\tSTATUS Pin: {}",status.is_set_high());

        let shld = shld_pin.lock().await;
        println!("\tSHLD Pin: {}",shld.is_set_high());

        let clk = clk_pin.lock().await;
        println!("\tCLK Pin: {}",clk.is_set_high());

        let data = data_pin.lock().await;
        println!("\tDATA Pin: {}",data.is_high());

        Ok(warp::reply::with_status(format!("Logged status of relevant GPIO pins."), StatusCode::OK))
    }

    /// GET /balls
    /// 
    /// Will return the current state of all ball contact switches (75 in total).  This is done
    /// by querying the GPIO that is connected to the PISO Shift Registers.
    /// 
    /// TODO: Implement in hardware, for now this just returns 75 static bits that change only slightly
    ///       to mimic a game in progress.
    pub async fn get_ball_states(pin: OutPin) -> Result<impl warp::Reply, Infallible> { 
        let mut status_pin = pin.lock().await;

        let array_size = 75;
        let mut ball_states = Vec::with_capacity(array_size);
        let mut rng = rand::thread_rng();

        for _ in 0..array_size {
            let this_bit: u8 = rng.gen_range(0..=1);
            ball_states.push(this_bit);
        }
        
        // Ok(warp::reply::json(&ball_states))
        Ok(warp::reply::with_status(json(&ball_states),  StatusCode::OK))
    }

}


