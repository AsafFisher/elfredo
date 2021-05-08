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
    size: usize,
}

const SIZE_FIELD_SIZE: usize = std::mem::size_of::<usize>();
const CHECK_START_OFFSET: usize = std::mem::size_of::<DataEntryHeader>() - SIZE_FIELD_SIZE;

impl DataEntryHeader {
    pub fn get_data(&self) -> Result<&'static [u8], ElfReadoError> {
        unsafe {
            if &self.magic != EMBEDDED_MAGIC {
                return Err(ElfReadoError::MagicNotFound {
                    magic_found: self.magic,
                });
            }
            if !self.is_crc_valid() {
                return Err(ElfReadoError::InvalidCRC {});
            }
            Ok(std::slice::from_raw_parts(
                (self as *const Self as *const u8).add(std::mem::size_of::<DataEntryHeader>()),
                self.size,
            ))
        }
    }
    unsafe fn is_crc_valid(&self) -> bool {
        crc32::checksum_ieee(std::slice::from_raw_parts(
            (self as *const Self as *const u8).add(CHECK_START_OFFSET),
            SIZE_FIELD_SIZE + self.size,
        )) == self.checksum
    }

    pub fn generate_entry(data: Vec<u8>) -> Result<Vec<u8>, Error> {
        let entry = DataEntryHeader {
            magic: *EMBEDDED_MAGIC,
            checksum: 0,
            size: data.len(),
        };

        let mut entry_vec = bincode::serialize(&entry)?;
        entry_vec.extend(&data);

        // Calculate and update checksum
        let checksum = crc32::checksum_ieee(&entry_vec[CHECK_START_OFFSET..]);
        // No need for check
        let dat_safe = unsafe { &mut *(entry_vec.as_ptr() as *mut DataEntryHeader) };
        dat_safe.checksum = checksum;
        Ok(entry_vec)
    }
}
