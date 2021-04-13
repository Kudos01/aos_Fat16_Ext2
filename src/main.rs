extern crate clap;
use clap::{App, Arg};
mod info;

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

    // TODO Check if file exists and throw an error if it doesn't
    // Also don't continue the program
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let operation = matches.value_of("operation");
    match operation {
        None => println!("No operation passed!"),
        Some(s) => match s {
            "/info" => {
                println!("Operation: {}", s);
                //if selected option is info, run the function that gets
                info::get_file_info(myfile);
            }
            _ => println!("Invalid operation {}", s),
        },
    }
}
