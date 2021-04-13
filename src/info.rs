//use positioned_io::ReadAt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;
use std::io::SeekFrom;

// Use the `pub` modifier to override default visibility.
pub fn get_file_info(file_name: &str) {
    //println!("file inside the function {}", file_name);

    let mut opened_file = match File::open(&file_name) {
        Err(why) => panic!("couldn't open {}: {}", file_name, why),
        Ok(opened_file) => opened_file,
    };

    /*
    let mut s = String::new();

    match opened_file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", file_name, why),
        Ok(_) => print!("{} contains:\n{}", file_name, s),
    }
    */

    // TODO? get the bytes of the file before attempting to read at offset?

    opened_file.seek(SeekFrom::Start(32)).unwrap();

    let buf: &mut [u8] = &mut [0; 4];

    let bytes_read = opened_file.read_exact(buf);

    println!("{:?} {:?}", buf, bytes_read);

    // TODO Check if FS is ext2 or FAT16 or neither
    // TODO If neither, then print error and return
}
