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
    pub free_inodes: u32,
    pub inode_size: u16,
    pub free_blocks_count: u32,
    pub block_size: u32,
    pub s_log_block_size: u32,
    pub reserved_blocks_count: u32,
    pub num_blocks: u32,
    pub first_data_block: u32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
}

struct DirEntry {
    pub inode: [u8; 4],
    pub rec_len: [u8; 2],
    pub name_len: [u8; 1],
    pub file_type: [u8; 1],
    pub name: Vec<u8>,
}

impl Default for DirEntry {
    fn default() -> DirEntry {
        DirEntry {
            inode: [0; 4],
            rec_len: [0; 2],
            name_len: [0; 1],
            file_type: [0; 1],
            name: Vec::new(),
        }
    }
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
            s_log_block_size: 0,
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
        let free_inodes_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 16 + 1024, free_inodes_temp).unwrap();
        self.free_inodes = LittleEndian::read_u32(&free_inodes_temp);

        // ------------------------ BLOCK SIZE ------------------------
        // starts at 24 + 1024
        let block_size_tmp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, 1024 + 24, block_size_tmp).unwrap();
        self.block_size = 1024 << LittleEndian::read_u32(&block_size_tmp);
        self.s_log_block_size = LittleEndian::read_u32(&block_size_tmp);

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

    fn find(self: &mut Self, file_to_find: &str, name_of_file: &str) -> &mut dyn Filesystem {
        let mut opened_file = match File::open(&name_of_file) {
            Err(why) => panic!("couldn't open {}: {}", name_of_file, why),
            Ok(opened_file) => opened_file,
        };

        find_file(self, &mut opened_file, 2, file_to_find);
        return self;
    }
}

fn find_file(ext2: &Ext2, opened_file: &File, inode: u32, file_to_find: &str) {
    let offset_inode = get_inode_offset(ext2, opened_file, inode);

    let first_data_block = get_first_data_block(opened_file, offset_inode);

    //Lastly, Read the data at the start of the block until we find 0's for the rec len
    let mut dir_entry: DirEntry = DirEntry::default();

    let data_offset: u64 = (first_data_block * ext2.block_size as u64).into();

    let num_data_blocks = get_data_blocks(ext2, opened_file, offset_inode);

    let mut bytes_read: u64 = 0;

    let mut found = 0;

    loop {
        fill_dir_entry(&opened_file, data_offset, bytes_read, &mut dir_entry);

        println!(
            "inode: {:?} rec len: {} name_len: {:?} file_type: {:?} NAME: {:?}\n",
            dir_entry.inode,
            LittleEndian::read_u16(&dir_entry.rec_len),
            dir_entry.name_len,
            dir_entry.file_type,
            str::from_utf8(&dir_entry.name)
        );
        if file_to_find.eq_ignore_ascii_case(str::from_utf8(&dir_entry.name).unwrap())
            && dir_entry.file_type[0] != 2
        {
            found = 1;

            let offset_inode_file =
                get_inode_offset(ext2, opened_file, LittleEndian::read_u32(&dir_entry.inode));

            println!(
                "offset: {}, inode: {}",
                offset_inode_file,
                LittleEndian::read_u32(&dir_entry.inode)
            );

            let size_file: &mut [u8] = &mut [0; 4];
            utilities::seek_read(opened_file, offset_inode_file + 4, size_file).unwrap();

            let blocks_data_file: &mut [u8] = &mut [0; 4];
            utilities::seek_read(opened_file, offset_inode_file + 40, blocks_data_file).unwrap();

            println!(
                "Blocks data of file: {}",
                LittleEndian::read_u32(blocks_data_file)
            );

            println!(
                "Found the file! File size: {}",
                LittleEndian::read_u32(size_file)
            );

            break;
        } else if dir_entry.file_type[0] == 2
            && str::from_utf8(&dir_entry.name).unwrap().ne("lost+found")
            && str::from_utf8(&dir_entry.name).unwrap().ne(".")
            && str::from_utf8(&dir_entry.name).unwrap().ne("..")
        {
            //println!("NOT LOST+FOUND! OR . OR ..");

            find_file(
                ext2,
                opened_file,
                LittleEndian::read_u32(&dir_entry.inode),
                file_to_find,
            );

            bytes_read = bytes_read + (LittleEndian::read_u16(&dir_entry.rec_len) as u64);

            if bytes_read >= (ext2.block_size as u64 * num_data_blocks).into() {
                break;
            }
        } else {
            bytes_read = bytes_read + (LittleEndian::read_u16(&dir_entry.rec_len) as u64);

            if bytes_read >= (ext2.block_size as u64 * num_data_blocks).into() {
                break;
            }
        }
    }
    if found == 0 {
        println!("File not found");
    }
}

fn get_first_data_block(opened_file: &File, inode_offset: u64) -> u64 {
    let first_data_block_temp: &mut [u8] = &mut [0; 4];
    utilities::seek_read(opened_file, inode_offset + 40, first_data_block_temp).unwrap();

    return LittleEndian::read_u32(&first_data_block_temp).into();
}

fn get_size(opened_file: &File, inode_offset: u64) -> u64 {
    let size_tmp: &mut [u8] = &mut [0; 4];
    utilities::seek_read(opened_file, inode_offset + 4, size_tmp).unwrap();

    return LittleEndian::read_u32(&size_tmp).into();
}

fn get_data_blocks(ext2: &Ext2, opened_file: &File, inode_offset: u64) -> u64 {
    let size_tmp: &mut [u8] = &mut [0; 4];
    utilities::seek_read(opened_file, inode_offset + 28, size_tmp).unwrap();

    return LittleEndian::read_u32(&size_tmp) as u64 / (2 << ext2.s_log_block_size);
}

fn fill_dir_entry(opened_file: &File, data_offset: u64, bytes_read: u64, dir_entry: &mut DirEntry) {
    utilities::seek_read(
        opened_file,
        data_offset + 7 + (bytes_read),
        &mut dir_entry.file_type,
    )
    .unwrap();

    utilities::seek_read(
        opened_file,
        data_offset + (bytes_read),
        &mut dir_entry.inode,
    )
    .unwrap();

    utilities::seek_read(
        opened_file,
        data_offset + 4 + (bytes_read),
        &mut dir_entry.rec_len,
    )
    .unwrap();
    utilities::seek_read(
        opened_file,
        data_offset + 6 + (bytes_read),
        &mut dir_entry.name_len,
    )
    .unwrap();

    //make a buffer of size of the name length
    dir_entry.name = vec![0; dir_entry.name_len[0].into()];

    utilities::seek_read(
        opened_file,
        data_offset + 8 + (bytes_read),
        &mut dir_entry.name,
    )
    .unwrap();
}

fn get_inode_offset(ext2: &Ext2, opened_file: &File, inode: u32) -> u64 {
    //First, use the root inode (first inode) and convert it to inode index
    let local_inode_index = (inode - 1) % ext2.inodes_per_group;
    //println!("local inode index: {}\n", local_inode_index);
    //Second, get what block group it is in (will always be 0 for the root inode)
    let block_group = (inode - 1) / ext2.inodes_per_group;
    //println!("BG: {}\n", block_group);

    //Third, get offset of the inode table this inode is in
    let offset_bg: u64 =
        ((block_group * ext2.block_size * ext2.blocks_per_group) + (2048 + 8)).into();
    //println!("offset_bg: {}\n", offset_bg);

    //Fourth, go to this @ and read 4 bytes to get the @ of the inode table for this BG
    let inode_table_temp: &mut [u8] = &mut [0; 4];
    utilities::seek_read(opened_file, offset_bg, inode_table_temp).unwrap();
    let inode_table_block = LittleEndian::read_u32(&inode_table_temp);
    //println!("inode table block: {}\n", inode_table_block);

    //Fifth, jump to the inode table and inode we were looking for and get the first i_block offset
    let offset_inode: u64 = ((inode_table_block * ext2.block_size)
        + (ext2.inode_size as u32 * local_inode_index))
        .into();

    println!("offset inode: {}\n", offset_inode);

    return offset_inode;
}
