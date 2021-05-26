# AOS_TheShooter_2021 #
By Felipe Perez Stoppa

# Quick start #

In order to install rust, follow the instructions found in the following link: 
https://doc.rust-lang.org/book/ch01-01-installation.html

# Running the code #
Once you have cloned the repository, you can run `cargo build` to build the project.

Once built, you can find the executable under `/repo_dir/target/debug/executable`

# Explanation of the practice #
For this project we were required to develop a program that would show the information, find and delete files of a volume formatted in one of 2 filesystems, FAT16 or EXT2.

### Design ###
For the design of the project, I decide to modulate the project files such that I would have the specific implementation of how to find, delete and get the info from the volumes in one file for each filesystem. The files `fat16.rs` and `ext2.rs` are the implementations of the functionality that a "filesystem" needs to have. The definition of what functionality a filesystem must have are is located in the `filesystem.rs` file. The utilities file contains the necesary functions to read and write at specific offsets in a file, amongst others. The `checker.rs` file is where the implementation for knowing what type of filesystem we are dealing with is.

### Data structures ###
The main data structures used were structs where the code saved the information from the file necesary for printing to the user in case of the `/info` option, and subsequently the information neceary to perform seek and read operations at different offsets knowing the structure of the file. Structs were also used to replicate the form of the directory entries of both filesystems.

### Tests performed ###
The tests performed consisted of running a combination of the following commands, while, in the case of find and delete, checking manually if the file existed by mounting the volume and navigating through all the directories of the test volumes provided. In the case of delete, once we knew the file existed, deleting it and checking after the operation of it still existed, and trying again once it was deleted to check if it didnt find a file to delete.

To see the info of a volume:
* `/repo_dir/target/debug/aos_the_shooter /info <VOLUME>`

To find a file in a volume:
* `/repo_dir/target/debug/aos_the_shooter /find <VOLUME> <FILE>`

To delete a file in a volume:
* `/repo_dir/target/debug/aos_the_shooter /delete <VOLUME> <FILE>`

### Temporal estimation ##
![temporal_est](/images/chart.png)

### Observed problems and conclusions ###
Most of the problems for this practice came from my lack of familiarity with rust, especially with crates, modules, traits and implementations. I had to change the way I thought about how to structure the program. Since in the past any big project that I had coded had been using the object oriented paradigm using classes and objects, while rust did not offer such a way of coding. Instead rust uses traits and implementations. For example, we can have a `walk` trait, defined. This is a behaviour. Then we can make a `human` and a `dog` struct, and make them both implement the `walk` trait. We then code for each of them how we want the walk behaviour to behave. For the dog we might say it uses 4 legs and for the human 2. It is the same trait, just different implementations. I took advantage of this with function return types. For the file checker function, I wanted it to return a filesystem of the type that it found the input file to be, either fat16 or ext2. This can be done by specifying that the return type be something that implements the `filesystem` trait. Granted, in my case I had to return a `Box` of a variable that implemented `filesystem`. This was because depending on wether the filesystem is ext2 or fat16, the size of the struct is different, and we have to "ask" rust to dynamically allocate memory depending on which one it is.

As for more technical problems with the project, the main thing was sitting down and understanding both filesystems to be able to perform the operations desired (finding files, deleting them, etc.).

# FAT16 #

### General info ###

FAT stands for file allocation table. It was originally develooped in 1977 for floppy disks. There exists several versions of the FAT file system, and the 32 bit versio, FAT32 is still currently used for most USB sticks and SD cards.

FAT16 is the 16 bit implementation of FAT, introduced in 1984.

### Boot sector ###
This sector goes by many names (0th sector for example), but the important fact is that it is the first sector of the volume, in the reserved region.

This sector contains information pertaining to the volume, like the bytes per sector, sectors per cluster, number of reserved sectors, number of FATs, etc.

### Reserved sectors ###
The first reserved sector is the boot sector, as mentioned before. However, there can be more than one reserved sector. For example, Microsoft uses sector 12 of the reserved sectors area for an extended boot loader.

### FAT Region ###
This region contains the mapping of which clusters are used by files and directories. 
### Root directory region ###
This region contains a Directory table, which is a special type of file that represents a directory, with the contents of the root directory. Every entry ender a directory table, is 32 bytes.

### Data Region ###
This is the region where the bulk of the space is taken up. It contains the file and directory data. 

### Directory entries ###
The directory entries in FAT have the following structure:
![fat16_dir](/images/fat16_dir.png)

The important thing to highlight here is the attribute byte:
![fat16_attr](/images/attribute_fat16.png)

This gives the type of file that this entry is describing.

* Read Only: self explanatory

* Hidden: This flag indicates to the system that the file should be hidden when doing normal directory listings.

* System: This flag indicates that the file/directory is important for the system

* Volume Name: When this flag is set, the directory entry is not pointing to a file, but to nothing. It is used for storing the volumme label

* Directory: This flag is set, when an entry in the directory table is not pointing to the beginning of a file, but to another directory table.

* Achieve Flag: This flag is used by backup utilities. The flag is set when ever a file is created, renamed or changed. Backup utilities can then use this flag to determine which files that has been modified since the last backup.

sources: 
* http://www.maverick-os.dk/FileSystemFormats/FAT16_FileSystem.html
* https://en.wikipedia.org/wiki/Design_of_the_FAT_file_system#Directory_table

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

* Inode number: The inode containing the information of this directory entry, which is 4 bytes long.
* Record length: The length of this directory entry. This is also used as an offset to know where we can find the next directory entry relative to the start offset of the current file. This field is 2 bytes long.
* Name length: 1 byte field indicaating the length of the name.
* File type: 1 byte field indicaating the type of the file (directory, regular file, character device, block device, etc.).
* Name: The name of this directory entry. It is a minimum of 4 bytes and a maximum of 256 bytes, as per the name length field.

sources: 
* https://piazza.com/class_profile/get_resource/il71xfllx3l16f/inz4wsb2m0w2oz#:~:text=The%20Ext2%20file%20system%20divides,lower%20average%20disk%20seek%20time.