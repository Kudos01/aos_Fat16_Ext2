# AOS_TheShooter_2021 #
By Felipe Perez Stoppa

# Quick start #

In order to install rust, follow the instructions found in the following link: 
https://doc.rust-lang.org/book/ch01-01-installation.html

# Running the code #
Once you have cloned the repository, you can run `cargo build` to build the project.

Once built, you can find the executable under `/repo_dir/target/debug/executable`


# FAT16 #

### General info ###

FAT stands for file allocation table.

# EXT2 #

### General info ###
EXT2 was created to replace ext fs, and added a variety of new features and performance improvements.

### inode ###
The inode (information node) is the basic building block of the ext2 file system. They contain all the information of every file and directory in the volume. The inodes are stored in an inode table, and there are different inode tables for each block group.

### Boot block ###
The boot block is the first block of the volume. It is of size 1KB and contains boot record information, if present.

### Superblock ###
The superblock is the block after the boot block, and the first block of every block group. It contains all the information about the configuration of the filesystem, like the total number of inodes and blocks in the filesystem and how many are free, how many inodes and blocks are in each block group, etc.

### Block groups ###
These are logical aprtitions of the disk, and the filesystem is structured as such to avoid fragmentation. Each block group contains a copy of the superblock as its first block. Furthermore, it contains the inode table for this particular block group. For example, if there are 1000 inodes per group (information we can get from the superblock), then inode 500 will be located in the inode table of block group 0, while inode 1500, will be located in the inode table of block group 1. The block groups also contain the data blocks that contain the files.

### Directory entries ###
The directory entries of ext2 have the following structure:
![ext2_dir](/images/ext2_dir.png)
source: https://piazza.com/class_profile/get_resource/il71xfllx3l16f/inz4wsb2m0w2oz#:~:text=The%20Ext2%20file%20system%20divides,lower%20average%20disk%20seek%20time.

* Inode number: The inode containing the information of this directory entry, which is 4 bytes long.
* Record length: The length of this directory entry. This is also used as an offset to know where we can find the next directory entry relative to the start offset of the current file. This field is 2 bytes long.
* Name length: 1 byte field indicaating the length of the name.
* File type: 1 byte field indicaating the type of the file (directory, regular file, character device, block device, etc.).
* Name: The name of this directory entry. It is a minimum of 4 bytes and a maximum of 256 bytes, as per the name length field.
