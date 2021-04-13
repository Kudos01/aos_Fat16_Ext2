extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("AOS The Shooter")
        .author("Felipe Perez <fpstoppa@gmail.com>")
        .arg(
            Arg::with_name("operation")
                .takes_value(true)
                .help("The desired operation"),
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("The volume to be scanned"),
        )
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let operation = matches.value_of("operation");
    match operation {
        None => println!("No operation passed!"),
        Some(s) => match s {
            "/info" => {
                println!("Operation: {}", s);
                //if selected option is info, run the function that gets
            }
            _ => println!("Invalid operation {}", s),
        },
    }
}