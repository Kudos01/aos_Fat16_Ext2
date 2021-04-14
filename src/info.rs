//use positioned_io::ReadAt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;
use std::io::SeekFrom;
/*
struct Checker {
    buf_fat: u16,
    buf_ext2: u16,
}

impl Default for Checker {
    fn default() -> Checker {
        Checker {
            buf_fat: 0,
            buf_ext2: 0,
        }
    }
}
*/

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
    //let checker = Checker::default();
    //Create a buffer of 2 bytes for reading to see if it is a Fat16 or 32
    let fat_buf: &mut [u8] = &mut [0; 2];
    //Create a buffer of 2 bytes for reading to see if it is a Fat16 or 32
    let ext2_buf: &mut [u8] = &mut [0; 2];

    //Start at 22 since this is BPB_FATSz16, if not 0, it is a FAT16 volume
    opened_file.seek(SeekFrom::Start(22)).unwrap();
    let _bytes_read = opened_file.read_exact(fat_buf);

    //For knowing if it is ext2, we check 2 bytes starting at offset 56 + 1024 (cus superblock)
    opened_file.seek(SeekFrom::Start(56 + 1024)).unwrap();
    let _bytes_read = opened_file.read_exact(ext2_buf);

    let fat_num = ((fat_buf[1] as u16) << 8) | fat_buf[0] as u16;
    let ext2_num = ((ext2_buf[1] as u16) << 8) | ext2_buf[0] as u16;

    //println!("FAT: {}", fat_num);
    //println!("EXT2: {}", ext2_num);
    //let checker = Checker::default();

    // Check if FS is ext2 or FAT16 or neither
    if ext2_num == 61267 {
        println!("EXT2 FS!");
        get_ext2_info();
    } else if fat_num == 16 {
        println!("FAT16 FS!");
        get_fat16_info();
    } else {
        //If neither, then print error and return
        println!("File system is neither EXT2 nor FAT16.");
    }
}

fn get_fat16_info() {}

fn get_ext2_info() {}
