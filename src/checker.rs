use crate::ext2::*;
use crate::fat16::*;
use crate::filesystem::*;
use crate::utilities::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;

//check exists
//check type (check at least size of the nearest offset)
// Perhaps change the Box implementation?
pub fn check_file(myfile: &str) -> Box<dyn Filesystem> {
    //check exists and if can be open it
    let mut opened_file = match File::open(&myfile) {
        Err(why) => panic!("couldn't open {}: {}", myfile, why),
        Ok(opened_file) => opened_file,
    };

    if opened_file.metadata().unwrap().len() < 56 + 1024 {
        panic!("File system is neither Fat16 nor Ext2");
    }
    //Create a buffer of 2 bytes for reading to see if it is a Fat16 or 32
    let fat_buf: &mut [u8] = &mut [0; 2];
    //Create a buffer of 2 bytes for reading to see if it is a Fat16 or 32
    let ext2_buf: &mut [u8] = &mut [0; 2];

    //Start at 22 since this is BPB_FATSz16, if not 0, it is a FAT16 volume
    utilities::seek_read(&mut opened_file, 22, fat_buf).unwrap();
    //For knowing if it is ext2, we check 2 bytes starting at offset 56 + 1024 (cus superblock)
    utilities::seek_read(&mut opened_file, 56 + 1024, ext2_buf).unwrap();

    //check what file it is
    // Check if FS is ext2 or FAT16 or neither
    if LittleEndian::read_u16(fat_buf) == 16 {
        //println!("EXT2 FS!");
        return Box::new(Fat16::default());
    } else if LittleEndian::read_u16(ext2_buf) == 61267 {
        //println!("FAT16 FS!");
        return Box::new(Ext2::default());
    } else {
        panic!("File system is neither Fat16 nor Ext2");
    }
}
