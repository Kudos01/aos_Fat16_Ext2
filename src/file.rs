use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Seek;
use std::io::{self, prelude::*, SeekFrom};

pub const EXT2_FLAG: i8 = 1;
pub const FAT16_FLAG: i8 = 2;
pub const FILE_ERROR_FLAG: i8 = -1;

#[derive(Default)]
pub struct Fat16Struct {
    pub volume_name: [u8; 8],
    pub sector_size: [u8; 2],
    pub sectors_per_cluster: [u8; 1],
    pub num_fats: [u8; 1],
    pub root_entries: [u8; 2],
    pub sectors_per_fat: [u8; 2],
    pub reserved_sectors: [u8; 2],
    pub volume_label: [u8; 11],
}

pub struct Ext2Struct {
    pub volume_name: [u8; 16],
    pub last_mounted: [u8; 4],
    pub last_check: [u8; 4],
    pub last_write: [u8; 4],
    pub num_inodes: [u8; 4],
    pub inodes_per_group: [u8; 4],
    pub first_inode: [u8; 4],
    pub free_inodes: [u8; 2],
    pub inode_size: [u8; 2],
    pub free_blocks_count: [u8; 4],
    pub block_size: u32,
    pub reserved_blocks_count: [u8; 4],
    pub num_blocks: [u8; 4],
    pub first_data_block: [u8; 4],
    pub blocks_per_group: [u8; 4],
    pub frags_per_group: [u8; 4],
}

impl Default for Ext2Struct {
    fn default() -> Ext2Struct {
        Ext2Struct {
            volume_name: [0; 16],
            last_mounted: [0; 4],
            last_check: [0; 4],
            last_write: [0; 4],
            num_inodes: [0; 4],
            inodes_per_group: [0; 4],
            first_inode: [0; 4],
            free_inodes: [0; 2],
            inode_size: [0; 2],
            free_blocks_count: [0; 4],
            block_size: 0,
            reserved_blocks_count: [0; 4],
            num_blocks: [0; 4],
            first_data_block: [0; 4],
            blocks_per_group: [0; 4],
            frags_per_group: [0; 4],
        }
    }
}

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
