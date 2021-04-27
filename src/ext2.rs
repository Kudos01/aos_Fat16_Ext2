use crate::filesystem::*;
use crate::utilities::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::str;

pub struct Ext2 {
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

impl Default for Ext2 {
    fn default() -> Ext2 {
        Ext2 {
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

impl Filesystem for Ext2 {
    fn load_info(&mut self, name: &str) -> &mut dyn Filesystem {
        let mut opened_file = match File::open(&name) {
            Err(why) => panic!("couldn't open {}: {}", name, why),
            Ok(opened_file) => opened_file,
        };

        // ------------------------ INODE SIZE ------------------------
        // starts at 88 + 1024
        utilities::seek_read(&mut opened_file, 1024 + 88, &mut self.inode_size).unwrap();

        // ------------------------ NUM INODES ------------------------
        // starts at 0 + 1024
        utilities::seek_read(&mut opened_file, 1024, &mut self.num_inodes).unwrap();

        // ------------------------ FIRST INODE ------------------------
        // starts at 84 + 1024
        utilities::seek_read(&mut opened_file, 84 + 1024, &mut self.first_inode).unwrap();

        // ------------------------ INODES PER GROUP ------------------------
        // starts at 40 + 1024
        utilities::seek_read(&mut opened_file, 40 + 1024, &mut self.inodes_per_group).unwrap();

        // ------------------------ FREE INODES ------------------------
        // starts at 14 + 2048
        utilities::seek_read(&mut opened_file, 14 + 2048, &mut self.free_inodes).unwrap();

        // ------------------------ BLOCK SIZE ------------------------
        // starts at 24 + 1024
        let block_size_tmp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 1024 + 24, block_size_tmp).unwrap();
        self.block_size = 1024 << LittleEndian::read_u32(&block_size_tmp);

        // ------------------------ RESERVED BLOCKS ------------------------
        // starts at 8 + 1024
        utilities::seek_read(&mut opened_file, 8 + 1024, &mut self.reserved_blocks_count).unwrap();

        // ------------------------ FREE BLOCKS ------------------------
        // starts at 12 + 1024
        utilities::seek_read(&mut opened_file, 12 + 1024, &mut self.free_blocks_count).unwrap();

        // ------------------------ TOTAL BLOCKS ------------------------
        // starts at 4 + 1024
        utilities::seek_read(&mut opened_file, 4 + 1024, &mut self.num_blocks).unwrap();

        // ------------------------ FIRST DATA BLOCK ------------------------
        // starts at 20 + 1024
        utilities::seek_read(&mut opened_file, 20 + 1024, &mut self.first_data_block).unwrap();

        // ------------------------ GROUP BLOCKS ------------------------
        // starts at 32 + 1024
        utilities::seek_read(&mut opened_file, 32 + 1024, &mut self.blocks_per_group).unwrap();

        // ------------------------ FRAGS GROUP ------------------------
        // starts at 36 + 1024
        utilities::seek_read(&mut opened_file, 36 + 1024, &mut self.frags_per_group).unwrap();

        // ------------------------ VOLUME NAME ------------------------
        // starts at 120 + 1024
        utilities::seek_read(&mut opened_file, 1144, &mut self.volume_name).unwrap();

        // ------------------------ LAST CHECKED ------------------------
        // starts at 64 + 1024
        utilities::seek_read(&mut opened_file, 1024 + 64, &mut self.last_check).unwrap();

        // ------------------------ LAST MOUNTED ------------------------
        // starts at 44+ 1024
        utilities::seek_read(&mut opened_file, 1024 + 44, &mut self.last_mounted).unwrap();

        // ------------------------ LAST WRITE/EDIT ------------------------
        // starts at 48+ 1024
        utilities::seek_read(&mut opened_file, 1024 + 64, &mut self.last_write).unwrap();

        return self;
    }

    fn print_info(self: &mut Self) -> &mut dyn Filesystem {
        println!("\n------ Filesystem Information ------\n");
        println!("Filesystem: EXT2\n");
        println!("INFO INODE");
        println!("Size Inode: {}", LittleEndian::read_u16(&self.inode_size));
        println!("Num Inode: {}", LittleEndian::read_u32(&self.num_inodes));
        println!("First inode: {}", LittleEndian::read_u32(&self.first_inode));
        println!(
            "Inodes per group: {}",
            LittleEndian::read_u32(&self.inodes_per_group)
        );
        println!(
            "Free inodes: {}\n",
            LittleEndian::read_u16(&self.free_inodes)
        );

        println!("BLOCK INFO");
        println!("Block size: {}", self.block_size);
        println!(
            "Reserved blocks: {}",
            LittleEndian::read_u32(&self.reserved_blocks_count)
        );
        println!(
            "Free blocks: {}",
            LittleEndian::read_u32(&self.free_blocks_count)
        );

        println!("Total blocks: {}", LittleEndian::read_u32(&self.num_blocks));
        println!(
            "First data block: {}",
            LittleEndian::read_u32(&self.first_data_block)
        );
        println!(
            "Blocks per group: {}",
            LittleEndian::read_u32(&self.blocks_per_group)
        );
        println!(
            "Group frags: {}\n",
            LittleEndian::read_u32(&self.frags_per_group)
        );
        println!("INFO VOLUME");
        match str::from_utf8(&mut self.volume_name) {
            Ok(v) => println!("Volume Name: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!(
            "Last Checked: {}",
            utilities::convert_to_utc_time(self.last_check).format("%A %e %B %Y, %T"),
        );
        println!(
            "Last Mounted: {}",
            utilities::convert_to_utc_time(self.last_mounted).format("%A %e %B %Y, %T"),
        );
        println!(
            "Last Write: {}\n",
            utilities::convert_to_utc_time(self.last_write).format("%A %e %B %Y, %T"),
        );
        return self;
    }
}