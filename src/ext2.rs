#![allow(non_upper_case_globals)]
use crate::filesystem::*;
use crate::utilities::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::fs::OpenOptions;
use std::str;

const s_wtime: u64 = 1024 + 48;
const s_lastcheck: u64 = 1024 + 64;
const s_mtime: u64 = 1024 + 44;
const s_volume_name: u64 = 1024 + 120;
const s_inodes_count: u64 = 1024 + 0;
const s_inodes_per_group: u64 = 1024 + 40;
const s_first_ino: u64 = 1024 + 84;
const s_free_inodes_count: u64 = 1024 + 16;
const s_inode_size: u64 = 1024 + 88;
const s_free_blocks_count: u64 = 1024 + 12;
const s_log_block_size: u64 = 1024 + 24;
const s_r_blocks_count: u64 = 1024 + 8;
const s_blocks_count: u64 = 1024 + 4;
const s_first_data_block: u64 = 1024 + 20;
const s_blocks_per_group: u64 = 1024 + 32;
const s_frags_per_group: u64 = 1024 + 36;

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
        let inode_size_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, s_inode_size, inode_size_temp).unwrap();
        self.inode_size = LittleEndian::read_u16(&inode_size_temp);

        // ------------------------ NUM INODES ------------------------
        let num_inodes_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_inodes_count, num_inodes_temp).unwrap();
        self.num_inodes = LittleEndian::read_u32(&num_inodes_temp);

        // ------------------------ FIRST INODE ------------------------
        let first_inode_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_first_ino, first_inode_temp).unwrap();
        self.first_inode = LittleEndian::read_u32(&first_inode_temp);

        // ------------------------ INODES PER GROUP ------------------------
        let inodes_per_group_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_inodes_per_group, inodes_per_group_temp).unwrap();
        self.inodes_per_group = LittleEndian::read_u32(&inodes_per_group_temp);

        // ------------------------ FREE INODES ------------------------
        let free_inodes_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_free_inodes_count, free_inodes_temp).unwrap();
        self.free_inodes = LittleEndian::read_u32(&free_inodes_temp);

        // ------------------------ BLOCK SIZE ------------------------
        let block_size_tmp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_log_block_size, block_size_tmp).unwrap();
        self.block_size = 1024 << LittleEndian::read_u32(&block_size_tmp);
        self.s_log_block_size = LittleEndian::read_u32(&block_size_tmp);

        // ------------------------ RESERVED BLOCKS ------------------------
        let reserved_blocks_count_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(
            &mut opened_file,
            s_r_blocks_count,
            reserved_blocks_count_temp,
        )
        .unwrap();
        self.reserved_blocks_count = LittleEndian::read_u32(&reserved_blocks_count_temp);

        // ------------------------ FREE BLOCKS ------------------------
        let free_blocks_count_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(
            &mut opened_file,
            s_free_blocks_count,
            free_blocks_count_temp,
        )
        .unwrap();
        self.free_blocks_count = LittleEndian::read_u32(&free_blocks_count_temp);

        // ------------------------ TOTAL BLOCKS ------------------------
        let num_blocks_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_blocks_count, num_blocks_temp).unwrap();
        self.num_blocks = LittleEndian::read_u32(&num_blocks_temp);

        // ------------------------ FIRST DATA BLOCK ------------------------
        let first_data_block_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_first_data_block, first_data_block_temp).unwrap();
        self.first_data_block = LittleEndian::read_u32(&first_data_block_temp);

        // ------------------------ GROUP BLOCKS ------------------------
        let blocks_per_group_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_blocks_per_group, blocks_per_group_temp).unwrap();
        self.blocks_per_group = LittleEndian::read_u32(&blocks_per_group_temp);

        // ------------------------ FRAGS GROUP ------------------------
        let frags_per_group_temp: &mut [u8] = &mut [0; 4];
        utilities::seek_read(&mut opened_file, s_frags_per_group, frags_per_group_temp).unwrap();
        self.frags_per_group = LittleEndian::read_u32(&frags_per_group_temp);

        // ------------------------ VOLUME NAME ------------------------
        utilities::seek_read(&mut opened_file, s_volume_name, &mut self.volume_name).unwrap();

        // ------------------------ LAST CHECKED ------------------------
        utilities::seek_read(&mut opened_file, s_lastcheck, &mut self.last_check).unwrap();

        // ------------------------ LAST MOUNTED ------------------------
        utilities::seek_read(&mut opened_file, s_mtime, &mut self.last_mounted).unwrap();

        // ------------------------ LAST WRITE/EDIT ------------------------
        utilities::seek_read(&mut opened_file, s_wtime, &mut self.last_write).unwrap();

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

    fn find(
        self: &mut Self,
        file_to_find: &str,
        name_of_file: &str,
        delete_flag: bool,
    ) -> &mut dyn Filesystem {
        let mut opened_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&name_of_file)
        {
            Err(why) => panic!("couldn't open {}: {}", name_of_file, why),
            Ok(opened_file) => opened_file,
        };

        let offset_inode = get_inode_offset(self, &opened_file, 2);

        let num_data_blocks = get_data_blocks(self, &opened_file, offset_inode);

        let mut block_counter = 0;

        let mut found;
        loop {
            found = find_file(
                self,
                &mut opened_file,
                offset_inode,
                file_to_find,
                delete_flag,
                block_counter,
            );
            block_counter += 1;

            if num_data_blocks <= block_counter || found {
                break;
            }
        }

        if !found {
            println!("could not find the file :(");
        }

        return self;
    }
}

fn find_file(
    ext2: &Ext2,
    opened_file: &File,
    offset_inode: u64,
    file_to_find: &str,
    delete_flag: bool,
    block_counter: u64,
) -> bool {
    let data_block_offset = get_data_block_offset(opened_file, offset_inode, block_counter);

    //Lastly, Read the data at the start of the block until we find 0's for the rec len
    let mut dir_entry: DirEntry = DirEntry::default();

    let data_offset: u64 = (data_block_offset * ext2.block_size as u64).into();

    let mut bytes_read: u64 = 0;

    let mut index_offset = 0;
    let mut array_rec_len: [u16; 2] = [0xFF; 2];

    loop {
        fill_dir_entry(&opened_file, data_offset, bytes_read, &mut dir_entry);

        array_rec_len[index_offset % 2] = LittleEndian::read_u16(&dir_entry.rec_len);
        index_offset += 1;

        if file_to_find.eq_ignore_ascii_case(str::from_utf8(&dir_entry.name).unwrap())
            && dir_entry.file_type[0] != 2
        {
            let offset_inode_file =
                get_inode_offset(ext2, opened_file, LittleEndian::read_u32(&dir_entry.inode));

            let size_file = get_size(opened_file, offset_inode_file);

            if delete_flag == false {
                //println!("Blocks data of file: {}", block_counter);
                println!("Found the file! File size: {}", size_file);
            } else {
                if block_counter == 0 {
                    println!("We wanna delete first block");

                    //get rec length of current file and save it
                    let current_rec_len = LittleEndian::read_u16(&dir_entry.rec_len);
                    //get rec length of the previous file
                    let rec_len_prev = array_rec_len[index_offset % 2];

                    let sum = current_rec_len + rec_len_prev;

                    utilities::seek_write(
                        opened_file,
                        (data_offset + bytes_read - rec_len_prev as u64 + 4).into(),
                        &mut sum.to_le_bytes(),
                    )
                    .unwrap();
                } else {
                    if array_rec_len[1] == 0xFF {
                        //The first file in the block
                        println!("File first in block");

                        //TODO Get the rec length of current file
                        let new_offset_from_base =
                            bytes_read + (LittleEndian::read_u16(&dir_entry.rec_len) as u64);
                        //TODO Go to next file and get its dir entry
                        let mut dir_entry_next: DirEntry = DirEntry::default();

                        fill_dir_entry(
                            &opened_file,
                            data_offset,
                            new_offset_from_base,
                            &mut dir_entry_next,
                        );

                        println!(
                            "inode: {:?} rec len: {:?} name: {:?} name len: {:?} file type: {:?}",
                            dir_entry.inode,
                            dir_entry.rec_len,
                            dir_entry.name,
                            dir_entry.name_len,
                            dir_entry.file_type
                        );
                        //TODO write the whole dir entry into the start offset of the current file
                        utilities::seek_write(
                            opened_file,
                            (data_offset + bytes_read).into(),
                            &mut dir_entry.inode,
                        )
                        .unwrap();

                        utilities::seek_write(
                            opened_file,
                            (data_offset + 4 + bytes_read).into(),
                            &mut dir_entry.rec_len,
                        )
                        .unwrap();

                        utilities::seek_write(
                            opened_file,
                            (data_offset + 6 + bytes_read).into(),
                            &mut dir_entry.name_len,
                        )
                        .unwrap();

                        utilities::seek_write(
                            opened_file,
                            (data_offset + 7 + bytes_read).into(),
                            &mut dir_entry.file_type,
                        )
                        .unwrap();

                        utilities::seek_write(
                            opened_file,
                            (data_offset + 8 + bytes_read).into(),
                            &mut dir_entry.name,
                        )
                        .unwrap();
                    } else {
                        println!("File NOT first in block");
                        //get rec length of current file and save it
                        let current_rec_len = LittleEndian::read_u16(&dir_entry.rec_len);
                        //get rec length of the previous file
                        let rec_len_prev = array_rec_len[index_offset % 2];

                        let sum = current_rec_len + rec_len_prev;

                        utilities::seek_write(
                            opened_file,
                            (data_offset + bytes_read - rec_len_prev as u64 + 4).into(),
                            &mut sum.to_le_bytes(),
                        )
                        .unwrap();
                    }
                }
            }

            return true;
        } else if dir_entry.file_type[0] == 2
            && str::from_utf8(&dir_entry.name).unwrap().ne("lost+found")
            && str::from_utf8(&dir_entry.name).unwrap().ne(".")
            && str::from_utf8(&dir_entry.name).unwrap().ne("..")
        {
            let mut inner_block_counter = 0;
            let mut found;

            //recalculate offset inode
            let offset_inode =
                get_inode_offset(ext2, &opened_file, LittleEndian::read_u32(&dir_entry.inode));
            let blocks_data_file = get_data_blocks(ext2, opened_file, offset_inode);
            println!("Blocks: {}", blocks_data_file);
            loop {
                //println!("inner: {}", inner_block_counter);
                found = find_file(
                    ext2,
                    opened_file,
                    offset_inode,
                    file_to_find,
                    delete_flag,
                    inner_block_counter,
                );
                inner_block_counter += 1;
                if blocks_data_file <= inner_block_counter || found {
                    break;
                }
            }

            bytes_read = bytes_read + (LittleEndian::read_u16(&dir_entry.rec_len) as u64);

            if bytes_read >= ext2.block_size as u64 || found {
                return found;
            }
        } else {
            bytes_read = bytes_read + (LittleEndian::read_u16(&dir_entry.rec_len) as u64);

            if bytes_read >= ext2.block_size as u64 {
                return false;
            }
        }
    }
}

fn get_data_block_offset(opened_file: &File, inode_offset: u64, block_counter: u64) -> u64 {
    let data_block_temp: &mut [u8] = &mut [0; 4];
    utilities::seek_read(
        opened_file,
        inode_offset + 40 + (block_counter * 4),
        data_block_temp,
    )
    .unwrap();

    return LittleEndian::read_u32(&data_block_temp).into();
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
    let inode_table_block =
        LittleEndian::read_u32(&inode_table_temp) + block_group * ext2.blocks_per_group;
    //println!("inode table block: {}\n", inode_table_block);

    //Fifth, jump to the inode table and inode we were looking for and get the first i_block offset
    let offset_inode: u64 = ((inode_table_block * ext2.block_size)
        + (ext2.inode_size as u32 * local_inode_index))
        .into();

    return offset_inode;
}
