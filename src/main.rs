extern crate clap;
mod checker;
mod ext2;
mod fat16;
mod filesystem;
mod utilities;
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
        .arg(
            Arg::with_name("file_to_find")
                .takes_value(true)
                .help("The file to find (for option /find)"),
        )
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("");
    let file_to_find = matches.value_of("file_to_find").unwrap_or("");
    let operation = matches.value_of("operation");

    match operation {
        None => println!("No operation passed!"),
        Some(s) => match s {
            "/info" => {
                //if selected option is info, run the function that gets
                checker::check_file(myfile)
                    .as_mut()
                    .load_info(myfile)
                    .print_info();
            }
            "/find" => {
                //if selected option is info, run the function that gets
                checker::check_file(myfile)
                    .as_mut()
                    .load_info(myfile)
                    .find(file_to_find, myfile);
            }
            _ => println!("Invalid operation {}", s),
        },
    }
}
