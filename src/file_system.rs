pub trait Filesystem {
    fn info(&self) -> None;
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

impl Filesystem for Ext2 {
    fn info(&self) -> None {
        println!("Inside the Ext2 implementation!")
    }
}

impl Filesystem for Fat16 {
    fn info(&self) -> None {
        println!("Inside the Fat16 implementation!")
    }
}
