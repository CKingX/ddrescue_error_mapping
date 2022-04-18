# ddrescue_error_mapping
This is a simple tool that converts ddrescue map files to dmintegrity table allowing you to use the bad sector maps on any data recovery tools that run under Linux (Current testing shows that this does *not* work under WSL). To create error mapping, you need both an image and the ddrescue map file. While ddrescue is the obvious tool that can create both, other tools can also create ddrescue compatible map files (like HDDSuperClone)

## Usage
Once you have an image, mount the image. Make sure to get the sector size (the default for mounting images is 512 bytes)
```
sudo losetup <device location ex: /dev/loop50> <path to image file> -r -b [512/4096]
```

This will create a block device at `<device location>` with the specified sector size in `-b` parameter. The `-r` parameter ensures the image is mounted read-only.

Once you have created the block device from the image, you can use the ddr_error_mapping by running 
```
ddr_error_mapping <ddrescue log file> <path of block device> >mapping.txt
```
Note the block device is the block device created with losetup. This command will create the necessary mapping for dmsetup. Finally, run `sudo cat <path to mapping> | sudo dmsetup create <device name>`

This will create a new device at `/dev/mapper/<device name>` which you can use with tools. Tested with DMDE (for faster recovery, set retries to 0), UFS Explorer and R-Studio

## Install
ddr_error_mapping install binaries are available at [Releases](https://github.com/CKingX/ddrescue_error_mapping/releases) page for Windows and Linux binaries (x64 architecture only). I currently do not have a Mac to test ddr_error_mapping with, but building and running it should work regardless. (Note that ddr_error_mapping will only create the device mapping file. You still need Linux to create the device mapper block device!)

## Build Guide
We can use cargo to build ddr_error_mapping. Currently tested with rustc version 1.60. To build, we first need to install rustup. For Windows, you can download rustup-init [here for Intel x64](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) and [here for Intel 32-bit](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe). For Windows, it will offer to download build tools automatically. For Linux (make sure to install build tools like build-essentials on Ubuntu) and macOS (XCode commandline tools will be required), run this command below instead to install rustup:
```
curl https://sh.rustup.rs -sSf | sh
```
Then, we can build:
```
git clone https://github.com/CKingX/ddrescue_error_mapping.git
cargo install --path ./ddrescue_error_mapping
```
Now you can run by typing ddrescue_error_mapping in terminal!

## Limitations
* `dmsetup` error device did not work under WSL in my testing
* Tested on Windows 10, Windows 11 (Note that I only tested my application ddr_error_mapping. Device mapper itself did not work under WSL in my testing), Ubuntu 20.04, and Ubuntu 18.04
