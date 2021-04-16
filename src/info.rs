use std::io::{self, prelude::*, SeekFrom};

use std::fs::File;
use std::io::Seek;
use std::str;

#[derive(Debug, Clone, Default)]
struct FileStruct {
    a1: [u8; 1],
    a2: [u8; 2],
}

fn seek_read(mut reader: impl Read + Seek, offset: u64, buf: &mut [u8]) -> io::Result<()> {
    reader.seek(SeekFrom::Start(offset))?;
    reader.read_exact(buf)?;
    Ok(())
}

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
        //get_ext2_info(opened_file);
    } else if fat_num == 16 {
        println!("FAT16 FS!");
        get_fat16_info(opened_file);
    } else {
        //If neither, then print error and return
        println!("File system is neither EXT2 nor FAT16.");
    }
}

fn get_fat16_info(mut opened_file: File) {
    println!("------ Filesystem Information ------");
    println!("Filesystem: FAT16");

    // ------------------------ VOLUME NAME ------------------------

    // Volume name starts at 3
    opened_file.seek(SeekFrom::Start(3)).unwrap();
    let aux: &mut [u8] = &mut [0; 8];
    let _buf = opened_file.read_exact(aux);

    match str::from_utf8(aux) {
        Ok(v) => println!("Volume Name: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // ------------------------ SIZE ------------------------

    let mut file_struct: FileStruct = Default::default();

    // starts at 11 BPB_BytsPerSec
    seek_read(&mut opened_file, 13, &mut file_struct.a1).unwrap();
    seek_read(&mut opened_file, 14, &mut file_struct.a2).unwrap();

    println!("{:?}", file_struct);

    // ------------------------ SECTORS PER CLUSTER ------------------------

    // starts at 13
    opened_file.seek(SeekFrom::Start(13)).unwrap();
    let aux: &mut [u8] = &mut [0; 1];
    let _buf = opened_file.read_exact(aux);

    println!("Sectors per cluster: {}", aux[0]);

    // ------------------------ RESERVED SECTORS ------------------------

    // starts at 14
    opened_file.seek(SeekFrom::Start(14)).unwrap();
    let aux: &mut [u8] = &mut [0; 2];
    //let _buf = opened_file.read_exact(aux);
    let _buf = opened_file.read_exact(aux);
    /*
    println!(
        "Reserved sectors: {:?}",
        ((aux1[0] as u16) << 8) | aux1[1] as u16
    );
    */

    // ------------------------ VOLUME LABEL ------------------------

    // Volume Label starts at 43
    opened_file.seek(SeekFrom::Start(43)).unwrap();
    let aux: &mut [u8] = &mut [0; 11];
    let _buf = opened_file.read_exact(aux);

    match str::from_utf8(aux) {
        Ok(v) => println!("Volume Label: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}

/*
fn get_ext2_info(mut opened_file: File) {
    println!("------ Filesystem Information ------");
}
*/
