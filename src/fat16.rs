use crate::filesystem::*;
use crate::utilities::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::str;

pub struct Fat16 {
    pub volume_name: [u8; 8],
    pub sector_size: u16,
    pub sectors_per_cluster: u8,
    pub num_fats: u8,
    pub root_entries: u16,
    pub sectors_per_fat: u16,
    pub reserved_sectors: u16,
    pub volume_label: [u8; 11],
}

impl Default for Fat16 {
    fn default() -> Fat16 {
        Fat16 {
            volume_name: [0; 8],
            sector_size: 0,
            sectors_per_cluster: 0,
            num_fats: 0,
            root_entries: 0,
            sectors_per_fat: 0,
            reserved_sectors: 0,
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
        let sector_size_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 11, sector_size_temp).unwrap();
        self.sector_size = LittleEndian::read_u16(sector_size_temp);

        // ------------------------ SECTORS PER CLUSTER ------------------------
        // starts at 13
        let sectors_per_cluster_temp: &mut [u8] = &mut [0; 1];
        utilities::seek_read(&mut opened_file, 13, sectors_per_cluster_temp).unwrap();
        self.sectors_per_cluster = sectors_per_cluster_temp[0];

        // ------------------------ RESERVED SECTORS ------------------------
        // starts at 14
        let reserved_sectors_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 14, reserved_sectors_temp).unwrap();
        self.reserved_sectors = LittleEndian::read_u16(reserved_sectors_temp);

        // ------------------------ VOLUME LABEL ------------------------
        // Volume Label starts at 43
        utilities::seek_read(&mut opened_file, 43, &mut self.volume_label).unwrap();

        // ------------------------ NUM FATS ------------------------
        // starts at 16
        let num_fats_temp: &mut [u8] = &mut [0; 1];
        utilities::seek_read(&mut opened_file, 16, num_fats_temp).unwrap();
        self.num_fats = num_fats_temp[0];

        // ------------------------ ROOT ENTRIES ------------------------
        // starts at 17
        let root_entries_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 17, root_entries_temp).unwrap();
        self.root_entries = LittleEndian::read_u16(root_entries_temp);

        // ------------------------ SECOTRS PER FAT ------------------------
        // starts at 22
        let sectors_per_fat_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 22, sectors_per_fat_temp).unwrap();
        self.sectors_per_fat = LittleEndian::read_u16(sectors_per_fat_temp);

        return self;
    }

    fn print_info(self: &mut Self) -> &mut dyn Filesystem {
        println!("\n------ Filesystem Information ------\n");
        println!("Filesystem: FAT16\n");
        match str::from_utf8(&self.volume_name) {
            Ok(v) => println!("Volume Name: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("Size: {}", self.sector_size);

        println!("Sectors per cluster: {}", self.sectors_per_cluster);

        println!("Reserved sectors: {}", self.reserved_sectors);

        println!("Number of FATs: {}", self.num_fats);
        println!("Root entries: {}", self.root_entries);

        println!("Sectors per FAT: {}", self.sectors_per_fat);

        match str::from_utf8(&self.volume_label) {
            Ok(v) => println!("Volume Label: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        return self;
    }
}
