# ddr-mount
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping?ref=badge_shield)

This is a simple tool that mounts ddrescue images with their map files, allowing it to present I/O errors for any bad sectors or untried areas to any Linux recovery tool. (Current testing shows that this does *not* work under WSL). To mount an image, you need both an image and the ddrescue map file. While ddrescue is the obvious tool that can create both, other tools can also create ddrescue compatible map files (like HDDSuperClone)

## Usage
Once you have an image, mount the image. Make sure to get the sector size (the default for mounting images is 512 bytes, but modern hard drivs use 4096 bytes sector size)
```
sudo ddr-mount mount -i <path to image file> -m <path to map file> -b <sector size>
```

This will create a block device at /dev/mapper/ddrm# (The exact number will be printed) with the specified sector size in `-b` parameter. The image itself is mounted read-only, so there is no risk to changing the image file.

Once done, you can unmount the image:
```
sudo ddr-mount unmount <device name like drrm0>
```

Finally, you can list all the images that are mounted with
```
ddr-mount list
```

## Install
ddr_error_mapping install binaries are available at [Releases](https://github.com/CKingX/ddrescue_error_mapping/releases) page for Ubuntu binaries (x64 architecture only).

## Build Guide
We can use cargo to build ddr_error_mapping. Currently tested with rustc version 1.60. To build, we first need to install rustup. Make sure to install build tools like build-essentials on Ubuntu. Finally, run this command below instead to install rustup:
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
* dm-mount does not yet work under WSL
* Tested on Ubuntu 20.04, and Ubuntu 18.04. Currently, dm-mount does not work under WSL


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping?ref=badge_large)