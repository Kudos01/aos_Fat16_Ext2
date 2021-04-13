use positioned_io::ReadAt;
use std::fs::File;

// Use the `pub` modifier to override default visibility.
pub fn get_file_info(file_name: &str) {
    //println!("file inside the function {}", file_name);

    let opened_file = match File::open(&file_name) {
        Err(why) => panic!("couldn't open {}: {}", file_name, why),
        Ok(opened_file) => opened_file,
    };

    // TODO Check if FS is ext2 or FAT16 or neither
    // TODO If neither, then print error and return
}
