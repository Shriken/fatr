use std::error;
use std::fs;
use std::io::Read;
use std::path::Path;

use itertools::Itertools;

use fat;


/// Convert the provided filename into something acceptable (8+3)
fn filename_to_dos(file_path: String) -> Result<String,Box<error::Error>> {
    let file_name = match Path::new(&file_path).file_name() {
        Some(f) => f.to_string_lossy().into_owned(),
        None => {
            return Err(errorf!("Unable to determine DOS filename of {}", file_path));
        }
    };
    Ok(file_name)
}

pub fn add_file(args: &[String])
    -> Result<(), Box<error::Error>>
{
    expect_args!(args, 2);

    let file_name  = args[0].clone().to_string();
    let image_name = args[1].clone().to_string();
    let dos_file_name = match filename_to_dos(file_name.clone()) {
        Ok(f) => f,
        Err(e) => {
            panic!(format!("Error calculating DOS filename: {:?}", e));
        },
    };

    let mut image = fat::Image::from_file(image_name.clone())?;

    // Don't overwrite a preexisting file.
    if let Ok(_) = image.get_file_entry(file_name.clone()) {
        return Err(errorf!("file {} already exists", file_name));
    }

    // Ensure input file exists.
    let file = fs::File::open(file_name)?;

    // Create a root dir entry.
    let (entry, index) = image.create_file_entry(dos_file_name)?;

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
        try!(image.write_data_sector(entry_index, &chunk));
    }

    image.save_file_entry(entry, index)?;
    image.save(image_name)?;
    Ok(())
}
