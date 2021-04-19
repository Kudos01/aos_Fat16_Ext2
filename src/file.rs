use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Seek;
use std::io::{self, prelude::*, SeekFrom};

pub const EXT2_FLAG: i8 = 1;
pub const FAT16_FLAG: i8 = 2;
pub const FILE_ERROR_FLAG: i8 = -1;

pub fn seek_read(mut reader: impl Read + Seek, offset: u64, buf: &mut [u8]) -> io::Result<()> {
    reader.seek(SeekFrom::Start(offset))?;
    reader.read_exact(buf)?;
    Ok(())
}

pub fn check_file(mut opened_file: &File) -> i8 {
    //Create a buffer of 2 bytes for reading to see if it is a Fat16 or 32
    let fat_buf: &mut [u8] = &mut [0; 2];
    //Create a buffer of 2 bytes for reading to see if it is a Fat16 or 32
    let ext2_buf: &mut [u8] = &mut [0; 2];

    //Start at 22 since this is BPB_FATSz16, if not 0, it is a FAT16 volume
    seek_read(&mut opened_file, 22, fat_buf).unwrap();
    //For knowing if it is ext2, we check 2 bytes starting at offset 56 + 1024 (cus superblock)
    seek_read(&mut opened_file, 56 + 1024, ext2_buf).unwrap();

    // Check if FS is ext2 or FAT16 or neither
    if LittleEndian::read_u16(ext2_buf) == 61267 {
        //println!("EXT2 FS!");
        //get_ext2_info(opened_file);
        return EXT2_FLAG;
    } else if LittleEndian::read_u16(fat_buf) == 16 {
        //println!("FAT16 FS!");
        //get_fat16_info(opened_file);
        return FAT16_FLAG;
    } else {
        //If neither, return error
        return FILE_ERROR_FLAG;
    }
}
