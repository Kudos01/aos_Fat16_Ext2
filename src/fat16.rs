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
    pub total_sectors: u16,
}

struct DirEntry {
    pub name: [u8; 8],
    pub extension: [u8; 3],
    pub filesize: [u8; 4],
    pub file_type: [u8; 1],
    pub starting_cluster: [u8; 2],
}

impl Default for DirEntry {
    fn default() -> DirEntry {
        DirEntry {
            name: [0; 8],
            extension: [0; 3],
            filesize: [0; 4],
            file_type: [0; 1],
            starting_cluster: [0; 2],
        }
    }
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
            total_sectors: 0,
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

        // ------------------------ Total Sectors ------------------------
        // starts at 19
        let total_sectors_temp: &mut [u8] = &mut [0; 2];
        utilities::seek_read(&mut opened_file, 19, total_sectors_temp).unwrap();
        self.total_sectors = LittleEndian::read_u16(total_sectors_temp);

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

        println!("Total Sectors: {}", self.total_sectors);

        match str::from_utf8(&self.volume_label) {
            Ok(v) => println!("Volume Label: {}", v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        return self;
    }

    fn find(self: &mut Self, file_to_find: &str, name_of_file: &str) -> &mut dyn Filesystem {
        let mut opened_file = match File::open(&name_of_file) {
            Err(why) => panic!("couldn't open {}: {}", name_of_file, why),
            Ok(opened_file) => opened_file,
        };

        let offset_dir = (self.reserved_sectors * self.sector_size)
            + (self.num_fats as u16 * self.sectors_per_fat * self.sector_size);

        let data_region_offset = (self.reserved_sectors * self.sector_size)
            + (self.num_fats as u16 * self.sectors_per_fat * self.sector_size)
            + (self.root_entries * 32);

        let cluster_size = self.sectors_per_cluster as u16 * self.sector_size;

        let found = find_file(
            self,
            &mut opened_file,
            file_to_find,
            offset_dir,
            data_region_offset,
            cluster_size,
        );

        if !found {
            println!("could not find the file :(");
        }

        return self;
    }
}

fn find_file(
    fat16: &Fat16,
    mut opened_file: &File,
    file_to_find: &str,
    mut offset_dir: u16,
    data_region_offset: u16,
    cluster_size: u16,
) -> bool {
    let mut dir_entry: DirEntry = DirEntry::default();

    // every dir entry is 64 bytes
    // (actually its 32 bytes but idk why the next dir entry starts at 64), first one is poop, idk why

    loop {
        offset_dir += 64;
        //first 8 bytes is the name
        utilities::seek_read(opened_file, offset_dir as u64, &mut dir_entry.name).unwrap();

        //read the file type
        utilities::seek_read(
            opened_file,
            offset_dir as u64 + 11,
            &mut dir_entry.file_type,
        )
        .unwrap();

        if LittleEndian::read_u64(&dir_entry.name) == 0 || dir_entry.file_type[0] == 15 {
            return false;
        }

        println!("Name: {}", str::from_utf8(&dir_entry.name).unwrap());

        let mut name = utilities::remove_whitespace(str::from_utf8(&dir_entry.name).unwrap());

        // next 3 bytes is the extension
        utilities::seek_read(opened_file, offset_dir as u64 + 8, &mut dir_entry.extension).unwrap();

        let extension = utilities::remove_whitespace(str::from_utf8(&dir_entry.extension).unwrap());

        if extension.capacity() > 0 {
            name.push_str(".");
            name.push_str(&extension);
        }

        // Finally, read the starting cluster from dir entry
        utilities::seek_read(
            opened_file,
            offset_dir as u64 + 26,
            &mut dir_entry.starting_cluster,
        )
        .unwrap();

        //check if the directory flag is set
        if name.eq_ignore_ascii_case(file_to_find) && (dir_entry.file_type[0] & 16) != 16 {
            //NOT a directory
            println!("File Found!");
            // last 4 bytes is size (32 -4 is starting offset)
            utilities::seek_read(opened_file, offset_dir as u64 + 28, &mut dir_entry.filesize)
                .unwrap();

            println!(
                "File size is: {} bytes",
                LittleEndian::read_u32(&dir_entry.filesize)
            );
            return true;
        } else if (dir_entry.file_type[0] & 16) == 16 {
            //TODO, recalculate offset
            let dir_offset = cluster_size as u128
                * (LittleEndian::read_u16(&dir_entry.starting_cluster) - 2) as u128
                + data_region_offset as u128;

            println!("AAAAAA: {}", dir_offset);

            let found = find_file(
                fat16,
                &mut opened_file,
                file_to_find,
                offset_dir,
                data_region_offset,
                cluster_size,
            );

            if found == true {
                return true;
            }
        }
    }
}
