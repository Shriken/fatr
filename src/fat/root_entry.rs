pub struct RootEntry {
    pub filename:  [u8; 8],
    pub extension: [u8; 3],
    attrs: u8,
    pub reserved: u16,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    _ignore: u16,
    pub last_write_time: u16,
    pub last_write_date: u16,
    pub first_logical_cluster: u16,
    pub file_size: u32, // in bytes
}

enum RootEntryAttr {
    ReadOnly,
    Hidden,
    System,
    VolumeLabel,
    Subdir,
    Archive,
}

#[allow(dead_code)]
impl RootEntry {
    pub fn is_read_only(&self)    -> bool {
        self.attrs & 0x01 == 0x01
    }
    pub fn is_hidden(&self)       -> bool {
        self.attrs & 0x02 == 0x02
    }
    pub fn is_system(&self)       -> bool {
        self.attrs & 0x04 == 0x04
    }
    pub fn is_volume_label(&self) -> bool {
        self.attrs & 0x08 == 0x08
    }
    pub fn is_subdir(&self)       -> bool {
        self.attrs & 0x10 == 0x10
    }
    pub fn is_archive(&self)      -> bool {
        self.attrs & 0x20 == 0x20
    }

    pub fn is_free(&self) -> bool {
        self.filename[0] == 0 || self.filename[0] == 0xe5
    }

    pub fn rest_are_free(&self) -> bool {
        self.filename[0] == 0
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
