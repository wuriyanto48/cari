extern crate cari;

use std::env;
use std::process;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use cari::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let config = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(1);
        } else {
            println!("{}: {}", program, err);
            process::exit(1);
        }
    });

    let (sender_res, receiver_res) = mpsc::channel();
    let (sender_contents, receiver_contents) = mpsc::channel();
    let shared_receiver_contents = Arc::new(Mutex::new(receiver_contents));

    match cari::run(config, sender_res, sender_contents, shared_receiver_contents) {
        Ok(_) => {},
        Err(err) => {
            println!("{}: {}", program, err);
            process::exit(1);
        },
    }

    if let Some(res) = receiver_res.iter().last() {
        println!("{}", res);
    };

}