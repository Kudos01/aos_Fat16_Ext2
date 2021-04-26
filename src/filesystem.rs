use crate::utilities::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::str;

pub trait Filesystem {
    fn load_info(&mut self, name: &str) -> &mut dyn Filesystem;
    fn print_info(&mut self) -> &mut dyn Filesystem;
}

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

pub struct Fat16 {
    pub volume_name: [u8; 8],
    pub sector_size: [u8; 2],
    pub sectors_per_cluster: [u8; 1],
    pub num_fats: [u8; 1],
    pub root_entries: [u8; 2],
    pub sectors_per_fat: [u8; 2],
    pub reserved_sectors: [u8; 2],
    pub volume_label: [u8; 11],
}

impl Default for Fat16 {
    fn default() -> Fat16 {
        Fat16 {
            volume_name: [0; 8],
            sector_size: [0; 2],
            sectors_per_cluster: [0; 1],
            num_fats: [0; 1],
            root_entries: [0; 2],
            sectors_per_fat: [0; 2],
            reserved_sectors: [0; 2],
            volume_label: [0; 11],
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

impl Filesystem for Fat16 {
    fn load_info(&mut self, name: &str) -> &mut dyn Filesystem {
        //having to open the file again is a bad solution, fix later
        println!("\n------ Filesystem Information ------\n");
        println!("Filesystem: FAT16\n");
        let mut opened_file = match File::open(&name) {
            Err(why) => panic!("couldn't open {}: {}", name, why),
            Ok(opened_file) => opened_file,
        };

        // ------------------------ VOLUME NAME ------------------------
        // Volume name starts at 3
        utilities::seek_read(&mut opened_file, 3, &mut self.volume_name).unwrap();

        // ------------------------ SIZE ------------------------
        // Volume name starts at 11
        utilities::seek_read(&mut opened_file, 11, &mut self.sector_size).unwrap();

        // ------------------------ SECTORS PER CLUSTER ------------------------
        // starts at 13
        utilities::seek_read(&mut opened_file, 13, &mut self.sectors_per_cluster).unwrap();

        // ------------------------ RESERVED SECTORS ------------------------
        // starts at 14
        utilities::seek_read(&mut opened_file, 14, &mut self.reserved_sectors).unwrap();

        // ------------------------ VOLUME LABEL ------------------------
        // Volume Label starts at 43
        utilities::seek_read(&mut opened_file, 43, &mut self.volume_label).unwrap();

        // ------------------------ NUM FATS ------------------------
        // starts at 16
        utilities::seek_read(&mut opened_file, 16, &mut self.num_fats).unwrap();

        // ------------------------ ROOT ENTRIES ------------------------
        // starts at 17
        utilities::seek_read(&mut opened_file, 17, &mut self.root_entries).unwrap();

        // ------------------------ SECOTRS PER FAT ------------------------
        // starts at 22
        utilities::seek_read(&mut opened_file, 22, &mut self.sectors_per_fat).unwrap();

        return self;
    }

    fn print_info(self: &mut Self) -> &mut dyn Filesystem {
        match str::from_utf8(&self.volume_name) {
            Ok(v) => println!("Volume Name: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("Size: {}", LittleEndian::read_u16(&self.sector_size));

        println!("Sectors per cluster: {}", self.sectors_per_cluster[0]);

        println!(
            "Reserved sectors: {}",
            LittleEndian::read_u16(&self.reserved_sectors)
        );

        match str::from_utf8(&self.volume_label) {
            Ok(v) => println!("Volume Label: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("Number of FATs: {}", self.num_fats[0]);
        println!(
            "Root entries: {}",
            LittleEndian::read_u16(&self.root_entries)
        );

        println!(
            "Sectors per FAT: {}",
            LittleEndian::read_u16(&self.sectors_per_fat)
        );

        return self;
    }
}

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
