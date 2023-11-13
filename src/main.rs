#![allow(unused)]

use std::io::{BufRead, BufReader};
use std::{fs::read_dir, io::Write};
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

fn list_from_file(file_path: String) -> Vec<(String, bool)> {
    let file = std::fs::File::open(file_path);
    let mut todo_list: Vec<(String, bool)> = Vec::new();

    if (file.is_ok()) {
        let file = file.unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line: String = line.unwrap();

            let is_checked = &line[0..line.find("] ").unwrap_or(line.len())];
            let is_checked =
                &is_checked[is_checked.find("[").unwrap_or(is_checked.len()) + 1..is_checked.len()];
            // check if is_checked is "X"
            let is_checked = is_checked == "X";

            let text = &line[line.find("] ").unwrap_or(line.len()) + 2..line.len()];

            println!("{} // {}", is_checked, text);

            let line = (text.to_string(), is_checked);
            todo_list.push(line);
        }
    }
    todo_list
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let my_args = std::env::args();

    // read todo list from file
    let mut todo_list: Vec<(String, bool)> = list_from_file(String::from("todo.rtd")); //Vec::new();


    let mut args = my_args.skip(1);

    if (args.len() == 0) {
        println!("No arguments passed");
        for (i, todo) in todo_list.iter().enumerate() {
            println!("{}: [{}] {}", i, if todo.1 { "X" } else { " " }, todo.0);
        }
        return Ok(());
    }

    let arg = args.next().unwrap();

    match arg {
        arg if arg == "delete" || arg == "del" => {
            let arg = args.collect::<Vec<String>>();
            let todo = todo_list.remove(arg[0].parse::<usize>().unwrap());
            println!("Deleting: {}", arg.join(" "));
        }
        arg if arg == "add" => {
            let arg = args.collect::<Vec<String>>();
            todo_list.push((arg.join(" "), false));
            println!("Adding: {}", arg.join(" "));
        }
        arg if arg == "check" => {
            let arg = args.collect::<Vec<String>>();
            println!("Checking: {}", arg[0].parse::<u8>().unwrap());
            let todo = todo_list.get_mut(arg[0].parse::<usize>().unwrap()).unwrap();
            todo.1 = !todo.1;
            println!("Checking: {}", arg.join(" "));
        }
        _ => {
            println!("Unknown argument: {}", arg);
        }
    }

    for (i, todo) in todo_list.iter().enumerate() {
        println!("{}: [{}] {}", i, if todo.1 { "X" } else { " " }, todo.0);
    }

    // save todo list to file
    let mut file = std::fs::File::create("todo.rtd")?;
    for (i, todo) in todo_list.iter().enumerate() {
        file.write_all(
            format!("{}: [{}] {}\n", i, if todo.1 { "X" } else { " " }, todo.0).as_bytes(),
        )?;
    }

    Ok(())
}
