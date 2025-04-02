// OK to ignore during early development, to be removed later
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(coerce_unsized)]

mod colors;
mod effects;
mod fixtures;
mod parameters;
mod show;
mod utils;

use joylight_backend::setup_logger;
use log::info;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    setup_logger();

    info!("Hello, world!");

    let mut interval1 = time::interval(Duration::from_secs(1));
    let mut interval2 = time::interval(Duration::from_millis(332));

    // task_interval.tick().await;

    let a = tokio::spawn(async move {
        loop {
            interval1.tick().await;
            info!("Tick");
        }
    });

    let b = tokio::spawn(async move {
        loop {
            interval2.tick().await;
            info!("Tick2");
        }
    });

    tokio::join!(a);
}
