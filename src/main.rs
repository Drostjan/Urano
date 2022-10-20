#![allow(unused_imports)]

mod urano;

use urano::Urano;
use crate::urano::task::Task;

extern crate clap;
extern crate home;

use clap::{App, Arg};
use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let _urano = Urano::new();
    let matches = App::new("Urano")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple TO-DO App ")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .help("Add a new task to the list of tasks")
                .takes_value(true)
                .empty_values(false),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("list of tasks")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("del")
                .help("Remove a task from the list of tasks")
                .takes_value(true)
                .empty_values(false),

        ).get_matches();

        println!("{:?}", matches.value_of("add"));
    
}


