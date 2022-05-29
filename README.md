# ddr-mount
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping?ref=badge_shield)
[![Rust](https://github.com/CKingX/ddrescue_error_mapping/actions/workflows/rust.yml/badge.svg)](https://github.com/CKingX/ddrescue_error_mapping/actions/workflows/rust.yml)

This is a simple tool that mounts ddrescue images with their map files, allowing it to present I/O errors for any bad sectors or untried areas to any Linux recovery tool. (Current testing shows that this does *not* work under WSL). To mount an image, you need both an image and the ddrescue map file. While ddrescue is the obvious tool that can create both, other tools can also create ddrescue compatible map files (like HDDSuperClone)

## Usage
Once you have an image, mount the image. Make sure to get the sector size (the default for mounting images is 512 bytes, but modern hard drives use 4096 bytes sector size)
```
sudo ddr-mount mount -i <path to image file> -m <path to map file> -b <sector size>
```

This will create a block device at /dev/mapper/ddrm# (The exact number will be printed) with the specified sector size in `-b` parameter. However, the sector size must be multiple of 512 bytes. The image itself is mounted read-only, so there is no risk to changing the image file.

Once done, you can unmount the image:
```
sudo ddr-mount unmount <device name like drrm0>
```
You can also unmount all images:
```
sudo ddr-mount unmount-all
```
Finally, you can list all the images that are mounted with
```
ddr-mount list
```

## Install
ddr-mount .deb file is available at [Releases](https://github.com/CKingX/ddrescue_error_mapping/releases) page for Ubuntu binaries (x64 architecture only). There is also a generic Linux executable file for 64-bit Intel systems that should run on most Linux distributions, provided `dmsetup` and `losetup` are installed. If you have rustup installed (see Build Guide), you can install by running
```
cargo install ddr-mount
```

Unless you build it yourself and move the bash and fish autocomplete and man page file manually (see Build Guide), or use the deb file, autocompletion and man files are not installed. However, you can either generate the man files and compleetion files by following the Build Guide, or manually install them by downloading them from releases and following the steps from Build Guide on installing them.

## Upgrade instructions
Before upgrade, make sure to unmount all images.

If previous version was installed using the deb package, first uninstall the older version
```
sudo apt remove ddr-mount
```
Then install ddr-mount using deb file as usual

If you used cargo to install, run `cargo install ddr-mount` again

If you used the ddr-mount linux binary, just replace it with the newer version. You may need to run `chmod +x <path to ddr-mount>` again

## Build Guide
We can use cargo to build ddr-mount. Currently tested with rustc version 1.60. To build, we first need to install rustup. Make sure to install build tools like build-essentials on Ubuntu. To begin, run this command below instead to install rustup:
```
curl https://sh.rustup.rs -sSf | sh
```
Then, we can build:
```
git clone https://github.com/CKingX/ddrescue_error_mapping.git
cargo install --path ./ddrescue_error_mapping
```
Now you can run by typing ddr-mount in terminal! Upgrading it is as simple as replacing cloning the repository again and running the cargo install command again.

If you would like to just build the binary, you can run this for debug binary:
```
cargo build
```
and this for release binary:
```
cargo build --release
```
You should find output in ./target/{debug/release}/ddr-mount

The bash-completion files should be found in ./target/{debug/release}/build/ddr-mount-{hash}/out/ddr-mount.bash (You may find multiple ddr-mount-{hash} folders. Sort by date modified, and then find by whichever folder has out folder inside). Copy the ddr-mount.bash file to /usr/share/bash-completion/completions/ and rename to ddr-mount to get tab complete in bash automatically. Fish completion files are also found and are called ddr-mount.fish.

Man files should also be found in the same out folder as `ddr-mount.bash`. Move it to /usr/share/man/man1/ddr-mount-{subcommands}.1

## Limitations
* ddr-mount does not yet work under WSL
* Tested on Ubuntu 20.04, and Ubuntu 18.04. Currently, ddr-mount does not work under WSL
* Can only mount images with a sector size that is a multiple of 512 bytes
* ddrescue on macOS will create a map file for a disk of 9223 PB as macOS does not report disk size. This will cause mount to fail. See [this](https://www.mail-archive.com/bug-ddrescue@gnu.org/msg02081.html) for more information and for potential solutions on working with ddrescue to create a functional map file.


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FCKingX%2Fddrescue_error_mapping?ref=badge_large)
