use std::error;
use std::fs;
use std::path;
use std::io::Read;

use itertools::Itertools;

use fat;

pub fn add_file(args: &[String])
    -> Result<(), Box<error::Error>>
{
    expect_args!(args, 2);

    let file_name  = args[0].clone();
    let image_name = args[1].clone();
    let fat_file_name = if args.len() > 2 {
        args[2].clone()
    } else {
        match path::Path::new(&file_name).file_name() {
            Some(n) => n.to_string_lossy().into_owned(),
            None => file_name.clone(),
        }
    };

    let mut image = fat::Image::from_file(image_name.clone())?;

    // Don't overwrite a preexisting file.
    if let Ok(_) = image.get_file_entry(file_name.clone()) {
        return Err(errorf!("file {} already exists", file_name));
    }

    // Ensure input file exists.
    let file = fs::File::open(file_name)?;
    let metadata = file.metadata()?;

    // Create a root dir entry.
    let (entry, index) =
        image.create_file_entry(fat_file_name, metadata.len() as u32)?;

    // Get free FAT entries, fill sectors with file data.
    for chunk in &file.bytes().chunks(image.sector_size()) {
        let chunk = chunk
            .map(|b_res| b_res.unwrap_or(0))
            .collect::<Vec<_>>();

        // Get free sector.
        let entry_index: usize;
        match image.get_free_fat_entry() {
            Some(i) => entry_index = i,
            None => {
                // TODO: Remove entries written so far.
                panic!("image ran out of space while writing file")
            },
        }

        // Write chunk.
        image.write_data_sector(entry_index, &chunk)?;
    }

    image.save_file_entry(entry, index)?;
    image.save(image_name)?;
    Ok(())
}
