//#[macro_use] extern crate log;
use log::{info, warn, error};

pub fn example(transport_name: &str) {
    let v = 42;
    info!("A friendly notice from {}: {}", transport_name, v);
    warn!(target: "Engine", "Low fuel in {}", transport_name);
    error!(target: "Engine", "{} is out of fuel", transport_name);
}