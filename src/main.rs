#![allow(unused)]

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

mod program;
use program::program::f1racer_run;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    f1racer_run().await;
    Ok(())
}
