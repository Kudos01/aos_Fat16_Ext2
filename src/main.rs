extern crate clap;
use clap::{App, Arg};
mod file;
mod info;

//test comment testing branch!

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

    //Check if file exists and throw an error if it doesn't
    // Also don't continue the program

    let operation = matches.value_of("operation");
    match operation {
        None => println!("No operation passed!"),
        Some(s) => match s {
            "/info" => {
                //if selected option is info, run the function that gets
                info::get_file_info(myfile);
            }
            "/find" => {
                //if selected option is info, run the function that gets
                //find::find_file(myfile);
            }
            _ => println!("Invalid operation {}", s),
        },
    }
}
