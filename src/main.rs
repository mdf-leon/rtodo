#![allow(unused)]

// use core::time;
use std::fs::read_dir;

use tokio::time;

use crate::prelude::*;

mod error;
mod prelude;
mod utils;

mod functions;
use functions::sleep_for::sleep_for_secs;

mod f1_racer;
use f1_racer::f1_racer::F1Racer;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let racer01 = f1_racer::f1_racer::F1Racer::new("Racer01".to_string(), 5);
    let racer02 = f1_racer::f1_racer::F1Racer::new("Racer02".to_string(), 5);
    let racer03 = f1_racer::f1_racer::F1Racer::new("Racer03".to_string(), 5);

    let handle01 = tokio::spawn(racer01);
    let handle02 = tokio::spawn(racer02);
    let handle03 = tokio::spawn(racer03);

    println!("Racer 01: {:?}", handle01.await);
    println!("Racer 02: {:?}", handle02.await);
    println!("Racer 03: {:?}", handle03.await);

    sleep_for_secs(1).await;
    Ok(())
}
