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
    pub num_inodes: u32,
    pub inodes_per_group: u32,
    pub first_inode: u32,
    pub free_inodes: u16,
    pub inode_size: u16,
    pub free_blocks_count: u32,
    pub block_size: u32,
    pub reserved_blocks_count: u32,
    pub num_blocks: u32,
    pub first_data_block: u32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
}

impl Default for Ext2 {
    fn default() -> Ext2 {
        Ext2 {
            volume_name: [0; 16],
            last_mounted: [0; 4],
            last_check: [0; 4],
            last_write: [0; 4],
            num_inodes: 0,
            inodes_per_group: 0,
            first_inode: 0,
            free_inodes: 0,
            inode_size: 0,
            free_blocks_count: 0,
            block_size: 0,
            reserved_blocks_count: 0,
            num_blocks: 0,
            first_data_block: 0,
            blocks_per_group: 0,
            frags_per_group: 0,
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
        let inode_size_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 1024 + 88, inode_size_temp).unwrap();
        self.inode_size = LittleEndian::read_u16(&inode_size_temp);

        // ------------------------ NUM INODES ------------------------
        // starts at 0 + 1024
        let num_inodes_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 1024, num_inodes_temp).unwrap();
        self.num_inodes = LittleEndian::read_u32(&num_inodes_temp);

        // ------------------------ FIRST INODE ------------------------
        // starts at 84 + 1024
        let first_inode_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 84 + 1024, first_inode_temp).unwrap();
        self.first_inode = LittleEndian::read_u32(&first_inode_temp);

        // ------------------------ INODES PER GROUP ------------------------
        // starts at 40 + 1024
        let inodes_per_group_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 40 + 1024, inodes_per_group_temp).unwrap();
        self.inodes_per_group = LittleEndian::read_u32(&inodes_per_group_temp);

        // ------------------------ FREE INODES ------------------------
        // starts at 16 + 1024
        let free_inodes_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 16 + 1024, free_inodes_temp).unwrap();
        self.free_inodes = LittleEndian::read_u16(&free_inodes_temp);

        // ------------------------ BLOCK SIZE ------------------------
        // starts at 24 + 1024
        let block_size_tmp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 1024 + 24, block_size_tmp).unwrap();
        self.block_size = 1024 << LittleEndian::read_u32(&block_size_tmp);

        // ------------------------ RESERVED BLOCKS ------------------------
        // starts at 8 + 1024
        let reserved_blocks_count_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 8 + 1024, reserved_blocks_count_temp).unwrap();
        self.reserved_blocks_count = LittleEndian::read_u32(&reserved_blocks_count_temp);

        // ------------------------ FREE BLOCKS ------------------------
        // starts at 12 + 1024
        let free_blocks_count_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 12 + 1024, free_blocks_count_temp).unwrap();
        self.free_blocks_count = LittleEndian::read_u32(&free_blocks_count_temp);

        // ------------------------ TOTAL BLOCKS ------------------------
        // starts at 4 + 1024
        let num_blocks_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 4 + 1024, num_blocks_temp).unwrap();
        self.num_blocks = LittleEndian::read_u32(&num_blocks_temp);

        // ------------------------ FIRST DATA BLOCK ------------------------
        // starts at 20 + 1024
        let first_data_block_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 20 + 1024, first_data_block_temp).unwrap();
        self.first_data_block = LittleEndian::read_u32(&first_data_block_temp);

        // ------------------------ GROUP BLOCKS ------------------------
        // starts at 32 + 1024
        let blocks_per_group_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 32 + 1024, blocks_per_group_temp).unwrap();
        self.blocks_per_group = LittleEndian::read_u32(&blocks_per_group_temp);

        // ------------------------ FRAGS GROUP ------------------------
        // starts at 36 + 1024
        let frags_per_group_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 36 + 1024, frags_per_group_temp).unwrap();
        self.frags_per_group = LittleEndian::read_u32(&frags_per_group_temp);

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
        println!("Size Inode: {}", self.inode_size);
        println!("Num Inode: {}", self.num_inodes);
        println!("First inode: {}", self.first_inode);
        println!("Inodes per group: {}", self.inodes_per_group);
        println!("Free inodes: {}\n", self.free_inodes);

        println!("BLOCK INFO");
        println!("Block size: {}", self.block_size);
        println!("Reserved blocks: {}", self.reserved_blocks_count);
        println!("Free blocks: {}", self.free_blocks_count);

        println!("Total blocks: {}", self.num_blocks);
        println!("First data block: {}", self.first_data_block);
        println!("Blocks per group: {}", self.blocks_per_group);
        println!("Group frags: {}\n", self.frags_per_group);
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
