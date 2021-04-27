use crate::filesystem::*;
use crate::utilities::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::str;

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

impl Filesystem for Fat16 {
    fn load_info(&mut self, name: &str) -> &mut dyn Filesystem {
        //having to open the file again is a bad solution, fix later
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
        println!("\n------ Filesystem Information ------\n");
        println!("Filesystem: FAT16\n");
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

        println!("Number of FATs: {}", self.num_fats[0]);
        println!(
            "Root entries: {}",
            LittleEndian::read_u16(&self.root_entries)
        );

        println!(
            "Sectors per FAT: {}",
            LittleEndian::read_u16(&self.sectors_per_fat)
        );

        match str::from_utf8(&self.volume_label) {
            Ok(v) => println!("Volume Label: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        return self;
    }
}
