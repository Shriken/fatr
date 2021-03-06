use std::error;
use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
#[repr(C)]
pub struct RootEntry {
    pub filename:  [u8; 8],
    pub extension: [u8; 3],
    attrs: u8,
    reserved: u16,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    pub hi_first_lcluster: u16,
    pub last_write_time: u16,
    pub last_write_date: u16,
    pub lo_first_lcluster: u16,
    pub file_size: u32, // in bytes
}

#[allow(dead_code)]
impl RootEntry {
    /// Create a new empty FAT root directory entry.
    pub fn new() -> RootEntry {
        RootEntry {
            filename:  [' ' as u8; 8],
            extension: [' ' as u8; 3],
            attrs: 0,
            reserved: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            hi_first_lcluster: 0,
            last_write_time: 0,
            last_write_date: 0,
            lo_first_lcluster: 0,
            file_size: 0,
        }
    }

    /// Get the filename
    pub fn filename(&self) -> Result<String, Box<error::Error>> {
        let mut my_fn = self.filename.to_vec();
        let mut name = my_fn
            .drain(..)
            .take_while(|&c| c != ' ' as u8)
            .collect::<Vec<u8>>();
        name.push('.' as u8);
        name.extend(self.extension.iter());

        match String::from_utf8(name) {
            Ok(s) => Ok(s),
            Err(err) => Err(From::from(err)),
        }
    }

    /// Set the filename
    pub fn set_filename(&mut self, filename: String)
        -> Result<(), Box<error::Error>>
    {
        let parts: Vec<_> = filename.split('.').collect();
        if parts.len() != 2 || parts[0].len() > 8 || parts[1].len() > 3 {
            return Err(From::from(format!("bad filename: \"{}\"", filename)));
        }

        // Pad out short filenames to proper length
        let new_fn = format!("{:8}", parts[0].to_uppercase());
        let new_ext = format!("{:3}", parts[1].to_uppercase());
        self.filename.copy_from_slice(new_fn.as_bytes());
        self.extension.copy_from_slice(new_ext.as_bytes());

        Ok(())
    }

    /// Set the file size
    pub fn set_size(&mut self, bytes: u32)
        -> Result<(), Box<error::Error>>
    {
        self.file_size = bytes;
        Ok(())
    }

    /// Gets the logical entry cluster
    pub fn entry_cluster(&self) -> u32 {
        (self.hi_first_lcluster as u32) << 16
            | self.lo_first_lcluster as u32
    }

    /// Sets the logical entry cluster
    pub fn set_entry_cluster(&mut self, cluster_num: u32)
        -> Result<(), Box<error::Error>>
    {
        self.lo_first_lcluster = (cluster_num & 0xFFFF) as u16;
        if cluster_num > u16::max_value() as u32 {
            // Only supported on FAT32
            self.hi_first_lcluster = (cluster_num >> 16) as u16;
        }
        Ok(())
    }

    pub fn is_read_only(&self)    -> bool { self.attrs & 0x01 == 0x01 }
    pub fn is_hidden(&self)       -> bool { self.attrs & 0x02 == 0x02 }
    pub fn is_system(&self)       -> bool { self.attrs & 0x04 == 0x04 }
    pub fn is_volume_label(&self) -> bool { self.attrs & 0x08 == 0x08 }
    pub fn is_subdir(&self)       -> bool { self.attrs & 0x10 == 0x10 }
    pub fn is_archive(&self)      -> bool { self.attrs & 0x20 == 0x20 }

    pub fn is_free(&self) -> bool {
        self.filename[0] == 0 || self.filename[0] == 0xe5
    }

    pub fn rest_are_free(&self) -> bool {
        self.filename[0] == 0
    }

    pub fn set_is_read_only(&mut self, on: bool) {
        self.attrs = (self.attrs & !0x01) | if on { 0x01 } else { 0 }
    }
    pub fn set_is_hidden(&mut self, on: bool) {
        self.attrs = (self.attrs & !0x02) | if on { 0x02 } else { 0 }
    }
    pub fn set_is_system(&mut self, on: bool) {
        self.attrs = (self.attrs & !0x04) | if on { 0x04 } else { 0 }
    }
    pub fn set_is_volume_label(&mut self, on: bool) {
        self.attrs = (self.attrs & !0x08) | if on { 0x08 } else { 0 }
    }
    pub fn set_is_subdir(&mut self, on: bool) {
        self.attrs = (self.attrs & !0x10) | if on { 0x10 } else { 0 }
    }
    pub fn set_is_archive(&mut self, on: bool) {
        self.attrs = (self.attrs & !0x20) | if on { 0x20 } else { 0 }
    }

    pub fn filename_full(&self) -> String {
        let filename = String::from_utf8(
            Vec::from(&self.filename[..])
        );
        let extension = String::from_utf8(
            Vec::from(&self.extension[..])
        );

        if filename.is_ok() && extension.is_ok() {
            format!(
                "{}.{}",
                filename.unwrap(),
                extension.unwrap()
            )
        } else {
            "BAD FILENAME".to_string()
        }
    }
}

impl Debug for RootEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RootEntry")
            .field(
                "filename",
                &self.filename().unwrap_or("bad filename".to_string())
            )
            .field("attrs",                 &self.attrs)
            .field("creation_time",         &self.creation_time)
            .field("creation_date",         &self.creation_date)
            .field("last_access_date",      &self.last_access_date)
            .field("hi_first_lcluster",     &self.hi_first_lcluster)
            .field("last_write_time",       &self.last_write_time)
            .field("last_write_date",       &self.last_write_date)
            .field("lo_first_lcluster",      &self.lo_first_lcluster)
            .field("file_size",             &format!("{:#x}", self.file_size))
            .finish()
    }
}
