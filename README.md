# fatr
A tool for manipulating FAT12, FAT16, and FAT32 images in native Rust.

## Example binary
```shell
# To build:
$ cargo build

# To run:
$ cargo run
```

## Crate usage example

```rust
extern crate fatr;

use fatr::fat;

fn main() {
    let image = fat::Image::from_file("/tmp/fat16.img").unwrap();
    println!(" Volume {}", image.volume_label().unwrap());
    println!(" Volume has {} bytes per sector\n", image.sector_size());
}
```
