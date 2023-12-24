# VIT - View Images in Terminal

[![Crates.io](https://img.shields.io/crates/v/vit.svg)](https://crates.io/crates/vit)
[![CI Status](https://github.com/BlackThunder080/vit/actions/workflows/rust.yml/badge.svg)](https://github.com/BlackThunder080/vit/actions/workflows/rust.yml)

## Install
Install using cargo
```
cargo install vit
```
or download from [releases](https://github.com/BlackThunder080/vit/releases)

## Dependancies
On linux you need x11 and openssl installed, to install on ubuntu or debian run
```
sudo apt-get install xorg-dev libssl-dev
```
Windows and Mac do not require any dependancies

## Usage
```
$ vit --help
Usage: vit.exe [OPTIONS] <FILE>

Arguments:
  <FILE>

Options:
  -c, --char-size <CHAR_SIZE>  [default: 1]
  -h, --help                   Print help
  -V, --version                Print version
```
`file` can be a path to a file or an image url

`char-size` is the number of characters per pixel, usually 2 or 3 works best for square pixels

## Example
```$ vit "https://cdn.pixabay.com/photo/2023/12/08/16/07/chocolate-8437801_1280.jpg" -c 2```
![Example Output](https://github.com/BlackThunder080/vit/blob/main/examples/low-res.png)

## Resolution
Resolution is dependant on terminal font size
![High Res Example](https://github.com/BlackThunder080/vit/blob/main/examples/high-res.png)

You can usually use mouse wheel scroll or trackpad pinch to set change font size
