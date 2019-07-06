
use std::process::Command;
use std::str;
use std::{thread, time};
use std::fs::File;
use std::fs;
use std::path::Path;
use std::path;

use clap::{Arg, App, ArgMatches};



fn main() {
    let matches = get_matches();
    let number_of_nodes = number_of_nodes(&matches);
    let number_of_writers = number_of_writers(&matches);
    let number_of_readers = number_of_readers(&matches);

    create_hosts_file(number_of_nodes);





    /*
    let node_id = 2;
    let color = "Green";








    let mut child = Command::new("/bin/bash")
                .arg("-c")
                .arg("cargo run --manifest-path ../application/Cargo.toml -- 2 hosts.txt Green")
                .spawn()
                .expect("failed to execute process");


    thread::sleep(time::Duration::from_millis(5000));
 
    child.kill();
    */
}

fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Local starter")
        .version("0.1")
        .author("Oskar Lundström")
        .about("Todo")

        .arg(Arg::with_name("number-of-nodes")
            .required(true)
            .help("The number of local nodes to run."))

        .arg(Arg::with_name("number-of-writers")
            .short("w")
            .long("number-of-writers")
            .help("The number of nodes that should write."))

        .arg(Arg::with_name("number-of-readers")
            .short("r")
            .long("number-of-readers")
            .help("The number of nodes that should read."))

        .get_matches()
}

fn number_of_nodes(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-nodes").unwrap().parse().unwrap()
}

fn number_of_writers(matches: &ArgMatches<'static>) -> i32 {
    if let Some(number_of_writers) = matches.value_of("number-of-writers") {
        number_of_writers.parse().unwrap()
    } else {
        0
    }
}

fn number_of_readers(matches: &ArgMatches<'static>) -> i32 {
    if let Some(number_of_readers) = matches.value_of("number-of-readers") {
        number_of_readers.parse().unwrap()
    } else {
        0
    }
}

fn create_hosts_file(number_of_nodes: i32) {
    let correct_string = hosts_file_string(number_of_nodes);
    let file_path = Path::new("hosts.txt");
    if file_path.exists() {
        if let Ok(existing_string) = fs::read_to_string(file_path) {
            if existing_string == correct_string {
                return;
            }
        }

        fs::remove_file(file_path).expect("Could not remove existing hosts.txt file");
    }

    fs::write(file_path, correct_string).expect("Could not write new hosts.txt file.");
}

fn hosts_file_string(number_of_nodes: i32) -> String {
    let mut string = String::new();
    let port_offset = 62000;

    for node_id in 1..number_of_nodes+1 {
        string.push_str(&format!("{},127.0.0.1:{}\n", node_id, node_id + port_offset));
    }

    string
}