use crate::file::*;

use std::fs::File;
use std::str;

use std::convert::TryInto;

use byteorder::{ByteOrder, LittleEndian};
use chrono::*;

// Use the `pub` modifier to override default visibility.
pub fn get_file_info(file_name: &str) {
    let opened_file = match File::open(&file_name) {
        Err(why) => panic!("couldn't open {}: {}", file_name, why),
        Ok(opened_file) => opened_file,
    };

    match check_file(&opened_file) {
        EXT2_FLAG => get_ext2_info(opened_file),
        FAT16_FLAG => get_fat16_info(opened_file),
        FILE_ERROR_FLAG => panic!("File system is neither EXT2 nor FAT16"),
        _ => panic!("We should not be here"),
    }
}

fn get_fat16_info(mut opened_file: File) {
    println!("\n------ Filesystem Information ------\n");
    println!("Filesystem: FAT16");

    // ------------------------ VOLUME NAME ------------------------

    //create a file struct to store the read information
    let mut fat16_struct: Fat16Struct = Default::default();

    // Volume name starts at 3
    seek_read(&mut opened_file, 3, &mut fat16_struct.volume_name).unwrap();

    match str::from_utf8(&fat16_struct.volume_name) {
        Ok(v) => println!("Volume Name: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // ------------------------ SIZE ------------------------
    // Volume name starts at 11
    seek_read(&mut opened_file, 11, &mut fat16_struct.sector_size).unwrap();
    println!("Size: {}", to_u16(&mut fat16_struct.sector_size, false));

    // ------------------------ SECTORS PER CLUSTER ------------------------

    // starts at 13
    seek_read(&mut opened_file, 13, &mut fat16_struct.sectors_per_cluster).unwrap();
    println!(
        "Sectors per cluster: {}",
        fat16_struct.sectors_per_cluster[0]
    );

    // ------------------------ RESERVED SECTORS ------------------------

    // starts at 14
    seek_read(&mut opened_file, 14, &mut fat16_struct.reserved_sectors).unwrap();
    println!(
        "Reserved sectors: {}",
        to_u16(&mut fat16_struct.reserved_sectors, false)
    );

    // ------------------------ VOLUME LABEL ------------------------

    // Volume Label starts at 43
    seek_read(&mut opened_file, 43, &mut fat16_struct.volume_label).unwrap();
    match str::from_utf8(&mut fat16_struct.volume_label) {
        Ok(v) => println!("Volume Label: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // ------------------------ NUM FATS ------------------------

    // starts at 16
    seek_read(&mut opened_file, 16, &mut fat16_struct.num_fats).unwrap();
    println!("Number of FATs: {}", fat16_struct.num_fats[0]);

    // ------------------------ ROOT ENTRIES ------------------------

    // starts at 17
    seek_read(&mut opened_file, 17, &mut fat16_struct.root_entries).unwrap();
    println!(
        "Root entries: {}",
        to_u16(&mut fat16_struct.root_entries, false)
    );

    // ------------------------ SECOTRS PER FAT ------------------------

    // starts at 22
    seek_read(&mut opened_file, 22, &mut fat16_struct.sectors_per_fat).unwrap();
    println!(
        "Sectors per FAT: {}",
        to_u16(&mut fat16_struct.sectors_per_fat, false)
    );
}

fn get_ext2_info(mut opened_file: File) {
    println!("\n------ Filesystem Information ------\n");
    println!("Filesystem: EXT2\n");
    let mut ext2_struct: Ext2Struct = Default::default();

    println!("INFO INODE");

    // ------------------------ INODE SIZE ------------------------
    // starts at 88 + 1024
    seek_read(&mut opened_file, 1024 + 88, &mut ext2_struct.inode_size).unwrap();
    println!(
        "Size Inode: {}",
        LittleEndian::read_u16(&ext2_struct.inode_size)
    );

    // ------------------------ NUM INODES ------------------------
    // starts at 0 + 1024
    seek_read(&mut opened_file, 1024, &mut ext2_struct.num_inodes).unwrap();
    println!(
        "Num Inode: {}",
        LittleEndian::read_u32(&ext2_struct.num_inodes)
    );

    // ------------------------ FIRST INODE ------------------------
    // starts at 84 + 1024
    seek_read(&mut opened_file, 84 + 1024, &mut ext2_struct.first_inode).unwrap();
    println!(
        "First inode: {}",
        LittleEndian::read_u32(&ext2_struct.first_inode)
    );

    // ------------------------ INODES PER GROUP ------------------------
    // starts at 40 + 1024
    seek_read(
        &mut opened_file,
        40 + 1024,
        &mut ext2_struct.inodes_per_group,
    )
    .unwrap();
    println!(
        "Inodes per group: {}",
        LittleEndian::read_u32(&ext2_struct.inodes_per_group)
    );

    // ------------------------ FREE INODES ------------------------
    // starts at 14 + 2048
    seek_read(&mut opened_file, 14 + 2048, &mut ext2_struct.free_inodes).unwrap();
    println!(
        "Free inodes: {}\n",
        LittleEndian::read_u16(&ext2_struct.free_inodes)
    );

    println!("BLOCK INFO");

    // ------------------------ BLOCK SIZE ------------------------
    // starts at 24 + 1024

    let block_size_tmp: &mut [u8] = &mut [0; 4];

    seek_read(&mut opened_file, 1024 + 24, block_size_tmp).unwrap();

    ext2_struct.block_size = 1024 << LittleEndian::read_u32(&block_size_tmp);

    println!("Block size: {}", ext2_struct.block_size);

    // ------------------------ RESERVED BLOCKS ------------------------
    // starts at 8 + 1024

    seek_read(
        &mut opened_file,
        8 + 1024,
        &mut ext2_struct.reserved_blocks_count,
    )
    .unwrap();

    println!(
        "Reserved blocks: {}",
        LittleEndian::read_u32(&ext2_struct.reserved_blocks_count)
    );

    // ------------------------ FREE BLOCKS ------------------------
    // starts at 12 + 1024
    seek_read(
        &mut opened_file,
        12 + 1024,
        &mut ext2_struct.free_blocks_count,
    )
    .unwrap();
    println!(
        "Free blocks: {}",
        LittleEndian::read_u32(&ext2_struct.free_blocks_count)
    );

    // ------------------------ TOTAL BLOCKS ------------------------
    // starts at 4 + 1024

    seek_read(&mut opened_file, 4 + 1024, &mut ext2_struct.num_blocks).unwrap();

    println!(
        "Total blocks: {}",
        LittleEndian::read_u32(&ext2_struct.num_blocks)
    );

    // ------------------------ FIRST DATA BLOCK ------------------------
    // starts at 20 + 1024

    seek_read(
        &mut opened_file,
        20 + 1024,
        &mut ext2_struct.first_data_block,
    )
    .unwrap();

    println!(
        "First data block: {}",
        LittleEndian::read_u32(&ext2_struct.first_data_block)
    );

    // ------------------------ GROUP BLOCKS ------------------------
    // starts at 32 + 1024

    seek_read(
        &mut opened_file,
        32 + 1024,
        &mut ext2_struct.blocks_per_group,
    )
    .unwrap();

    println!(
        "Blocks per group: {}",
        LittleEndian::read_u32(&ext2_struct.blocks_per_group)
    );

    // ------------------------ FRAGS GROUP ------------------------
    // starts at 36 + 1024

    seek_read(
        &mut opened_file,
        36 + 1024,
        &mut ext2_struct.frags_per_group,
    )
    .unwrap();

    println!(
        "Group frags: {}\n",
        LittleEndian::read_u32(&ext2_struct.frags_per_group)
    );

    println!("INFO VOLUME");
    // ------------------------ VOLUME NAME ------------------------
    // starts at 120 + 1024
    seek_read(&mut opened_file, 1144, &mut ext2_struct.volume_name).unwrap();
    match str::from_utf8(&mut ext2_struct.volume_name) {
        Ok(v) => println!("Volume Name: {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    // ------------------------ LAST CHECKED ------------------------
    // starts at 64 + 1024
    seek_read(&mut opened_file, 1024 + 64, &mut ext2_struct.last_check).unwrap();
    println!(
        "Last Checked: {}",
        convert_to_utc_time(ext2_struct.last_check).format("%A %e %B %Y, %T"),
    );

    // ------------------------ LAST MOUNTED ------------------------
    // starts at 44+ 1024
    seek_read(&mut opened_file, 1024 + 44, &mut ext2_struct.last_mounted).unwrap();
    println!(
        "Last Mounted: {}",
        convert_to_utc_time(ext2_struct.last_mounted).format("%A %e %B %Y, %T"),
    );

    // ------------------------ LAST WRITE/EDIT ------------------------
    // starts at 48+ 1024
    seek_read(&mut opened_file, 1024 + 64, &mut ext2_struct.last_write).unwrap();
    println!(
        "Last Write: {}\n",
        convert_to_utc_time(ext2_struct.last_write).format("%A %e %B %Y, %T"),
    );
}

fn to_u16(to_convert: &mut [u8; 2], little_endian: bool) -> u16 {
    match little_endian {
        true => return ((to_convert[0] as u16) << 8) | to_convert[1] as u16,
        false => return ((to_convert[1] as u16) << 8) | to_convert[0] as u16,
    }
}

fn convert_to_utc_time(to_convert: [u8; 4]) -> chrono::DateTime<chrono::Utc> {
    //convert unix time to current time
    let timestamp = LittleEndian::read_u32(&to_convert);
    let naive = NaiveDateTime::from_timestamp(timestamp.try_into().unwrap(), 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    // Format the datetime how you want
    return datetime;
}
