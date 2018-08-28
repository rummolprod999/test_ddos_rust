extern crate clap;
extern crate reqwest;

use clap::{Arg, App};
use std::io::Read;
use std::error::Error;
use std::process;

fn main() {
    let matches = App::new("Test dos Site")
        .version("0.1.0")
        .author("Alexander")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("site")
            .short("s")
            .long("site")
            .takes_value(true)
            .help("A site for dos check"))
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("number for check"))
        .get_matches();

    let site = match matches.value_of("site") {
        None => {
            println!("bad site, use -h for help");
            "";
            process::exit(0x0100);
        }
        Some(s) => s
    };

    let num_str = matches.value_of("num");
    let mut num = match num_str {
        None => {
            println!("bad number, use -h for help");
            0;
            process::exit(0x0100);
        }
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            }
        }
    };
    let mut done = false;
    while !done {
        num = num - 1;
        if num == 0 {
            done = true;
        }
        match checker(site) {
            Ok(()) => (),
            Err(e) => println!("error {}", e)
        }
    }
}

fn checker(site: &str) -> Result<(), Box<Error>> {
    let mut res = reqwest::get(site)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
    println!("Body:\n{}", body);
    Ok(())
}