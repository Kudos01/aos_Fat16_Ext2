use std::fs::File;
use std::io::Seek;
use std::io::{self, prelude::*, SeekFrom};
use std::str;

#[derive(Debug, Clone, Default)]
struct FileStruct {
    volume_name: [u8; 8],
    sector_size: [u8; 2],
    sectors_per_cluster: [u8; 1],
    num_fats: [u8; 1],
    root_entries: [u8; 2],
    sectors_per_fat: [u8; 2],
    reserved_sectors: [u8; 2],
    volume_label: [u8; 11],
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

    // TODO? get the bytes of the file before attempting to read at offset?
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

    //create a file struct to store the read information
    let mut file_struct: FileStruct = Default::default();

    // Volume name starts at 3
    seek_read(&mut opened_file, 3, &mut file_struct.volume_name).unwrap();

    match str::from_utf8(&file_struct.volume_name) {
        Ok(v) => println!("Volume Name: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // ------------------------ SIZE ------------------------
    // Volume name starts at 11
    seek_read(&mut opened_file, 11, &mut file_struct.sector_size).unwrap();
    println!("Size: {}", to_u16(&mut file_struct.sector_size, false));

    // ------------------------ SECTORS PER CLUSTER ------------------------

    // starts at 13
    seek_read(&mut opened_file, 13, &mut file_struct.sectors_per_cluster).unwrap();
    println!(
        "Sectors per cluster: {}",
        file_struct.sectors_per_cluster[0]
    );

    // ------------------------ RESERVED SECTORS ------------------------

    // starts at 14
    seek_read(&mut opened_file, 14, &mut file_struct.reserved_sectors).unwrap();
    println!(
        "Reserved sectors: {}",
        to_u16(&mut file_struct.reserved_sectors, false)
    );

    // ------------------------ VOLUME LABEL ------------------------

    // Volume Label starts at 43
    seek_read(&mut opened_file, 43, &mut file_struct.volume_label).unwrap();
    match str::from_utf8(&mut file_struct.volume_label) {
        Ok(v) => println!("Volume Label: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // ------------------------ NUM FATS ------------------------

    // starts at 16
    seek_read(&mut opened_file, 16, &mut file_struct.num_fats).unwrap();
    println!("Number of FATs: {}", file_struct.num_fats[0]);

    // ------------------------ ROOT ENTRIES ------------------------

    // starts at 17
    seek_read(&mut opened_file, 17, &mut file_struct.root_entries).unwrap();
    println!(
        "Root entries: {}",
        to_u16(&mut file_struct.root_entries, false)
    );

    // ------------------------ SECOTRS PER FAT ------------------------

    // starts at 22
    seek_read(&mut opened_file, 22, &mut file_struct.root_entries).unwrap();
    println!(
        "Sectors per FAT: {}",
        to_u16(&mut file_struct.root_entries, false)
    );
}

fn to_u16(to_convert: &mut [u8; 2], little_endian: bool) -> u16 {
    match little_endian {
        true => return ((to_convert[0] as u16) << 8) | to_convert[1] as u16,
        false => return ((to_convert[1] as u16) << 8) | to_convert[0] as u16,
    }
}

/*
fn get_ext2_info(mut opened_file: File) {
    println!("------ Filesystem Information ------");
}
*/
