use std::error;

use fat;

pub fn detail_file(args: &[String])
    -> Result<(), Box<error::Error>>
{
    expect_args!(args, 2);

    let image_fn = args[0].clone();
    let image = fat::Image::from_file(image_fn)?;

    let file_metadata = image.get_file_entry(args[1].clone())?;
    println!("{:#?}", file_metadata);

    // XXX: We ignore the u32 entry clusters for now and cast to u16.
    // In the future we need to convert everything to u32 for FAT32 support.

    let mut cluster_num = file_metadata.entry_cluster() as u16;
    const CLUSTER_NUMS_PER_LINE: usize = 8;
    'outer: loop {
        for _ in 0 .. CLUSTER_NUMS_PER_LINE {
            let next_cluster = image.get_fat_entry(cluster_num);
            print!("{:#x}\t", cluster_num);

            if !fat::cluster_num_is_valid(next_cluster) {
                println!("\n{:#x}", next_cluster);
                break 'outer;
            }

            cluster_num = next_cluster;
        }
        println!("");
    }

    Ok(())
}
