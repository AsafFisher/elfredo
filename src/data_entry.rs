//! Handles dynamic resource section (DRS)
//!
//! This module provides methods to generate and read DRS entry, DRS entry is a section dedicated
//! for post-compilation patching.
//! The section `.extended` holds the DRS entry that can be changed by `objcopy --update-section` or
//! `embeditor`

use bincode;
use crc::crc32;
use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Fail)]
pub enum ElfReadoError {
    #[fail(display = "Embedded magic was not found: {:?}", magic_found)]
    MagicNotFound { magic_found: [u8; 4] },
    #[fail(display = "Calculated CRC is invalid.")]
    InvalidCRC {},
}

pub const EMBEDDED_MAGIC: &[u8; 4] = b"\xDE\xAD\xBE\xEF";
#[link_section = ".extended"]
pub static EXTENDED_DATA: DataEntryHeader = DataEntryHeader {
    magic: [0, 0, 0, 0],
    checksum: 0,
    size: 0,
};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DataEntryHeader {
    magic: [u8; 4],
    checksum: u32,
    size: u64,
}

const CHECK_START_OFFSET: usize = std::mem::size_of::<DataEntryHeader>();

impl DataEntryHeader {
    /// Gets data from the embedded `EXTENDED_DATA` section. This data can be dynamically
    /// changed by applying embeditor to change it.
    /// This function is for internal use only.
    ///
    /// #Examples
    ///
    /// ```no_run
    /// # use elfredo::data_entry;
    /// # fn main() -> Result<(), data_entry::ElfReadoError>{
    ///      let str = String::from_utf8(
    ///         data_entry::EXTENDED_DATA.get_data::<&'static [u8]>()?.to_vec()).expect( "Not utf8");
    ///      println!("{}", str);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn get_data<'d, T: Deserialize<'d>>(&self) -> Result<T, ElfReadoError> {
        unsafe {
            if &self.magic != EMBEDDED_MAGIC {
                return Err(ElfReadoError::MagicNotFound {
                    magic_found: self.magic,
                });
            }
            if !self.is_crc_valid() {
                return Err(ElfReadoError::InvalidCRC {});
            }
            Ok(bincode::deserialize(std::slice::from_raw_parts(
                (self as *const Self as *const u8).add(std::mem::size_of::<DataEntryHeader>()),
                self.size as usize,
            ))
            .unwrap())
        }
    }

    pub fn ptr_to_data<'d, T: Deserialize<'d>>(ptr: &[u8]) -> Result<T, ElfReadoError> {
        let ptr = ptr.as_ptr() as *const DataEntryHeader;
        unsafe { &*ptr }.get_data::<T>()
    }

    unsafe fn is_crc_valid(&self) -> bool {
        crc32::checksum_ieee(std::slice::from_raw_parts(
            (self as *const Self as *const u8).add(CHECK_START_OFFSET),
            self.size as usize,
        )) == self.checksum
    }

    /// Generates a dynamic resource section that contains data.
    /// DRS structure:
    /// | MAGIC    |
    /// | checksum |
    /// | size     |
    /// | DATA     |
    ///
    /// #Examples
    ///
    /// ```no_run
    /// # use elfredo::data_entry::DataEntryHeader;
    /// # fn main() -> Result<(), failure::Error>{
    ///       let data_entry = DataEntryHeader::generate_entry(b"Hello".to_vec());
    ///       println!("{:?}", data_entry);
    ///       Ok(())
    /// # }
    /// ```
    pub fn generate_entry<T: Serialize>(data: T) -> Result<Vec<u8>, Error> {
        let entry = DataEntryHeader {
            magic: *EMBEDDED_MAGIC,
            checksum: 0,
            size: bincode::serialized_size(&data).unwrap(),
        };

        // Construct the buffer
        let mut entry_vec = bincode::serialize(&entry)?;
        entry_vec.extend(bincode::serialize(&data)?);

        let checksum = crc32::checksum_ieee(&entry_vec[CHECK_START_OFFSET..]);
        // No need for check
        let dat_safe = unsafe { &mut *(entry_vec.as_ptr() as *mut DataEntryHeader) };
        dat_safe.checksum = checksum;
        Ok(entry_vec)
    }
}
