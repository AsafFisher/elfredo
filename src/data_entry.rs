use crc::crc32;
use bincode;
use serde::{Deserialize, Serialize};
use crc::crc32::checksum_ieee;

pub const EMBEDDED_MAGIC: &[u8; 4] = b"\xDE\xAD\xBE\xEF";
#[link_section = ".extended"]
pub static extended_data: DataEntryHeader = DataEntryHeader {
    magic: [0, 0, 0, 0],
    checksum: 0,
    size: 0,
};


#[repr(C)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DataEntryHeader {
    magic: [u8; 4],
    checksum: u32,
    size: usize,
}

const SIZE_FIELD_SIZE: usize = std::mem::size_of::<usize>();
const CHECK_START_OFFSET: usize = std::mem::size_of::<DataEntryHeader>() - SIZE_FIELD_SIZE;

impl DataEntryHeader {
    pub fn get_data(&self) -> Result<&'static [u8], ()> {
        unsafe {
            if &self.magic == EMBEDDED_MAGIC {
                if self.is_crc_valid() {
                    return Ok(std::slice::from_raw_parts(
                        (self as *const Self as *const u8).add(std::mem::size_of::<DataEntryHeader>()),
                        self.size));
                }
            }
            Err(())
        }
    }
    unsafe fn is_crc_valid(&self) -> bool {
        crc32::checksum_ieee(
            std::slice::from_raw_parts(
                (self as *const Self as *const u8).add(CHECK_START_OFFSET),
                SIZE_FIELD_SIZE + self.size)
        ) == self.checksum
    }

    pub fn generate_entry(data: Vec<u8>) -> Result<Vec<u8>, ()> {
        let entry = DataEntryHeader {
            magic: EMBEDDED_MAGIC.clone(),
            checksum: 0,
            size: data.len(),
        };
        match bincode::serialize(&entry) {
            Ok(mut entry_vec) => {
                entry_vec.extend(&data);
                let checksum = crc32::checksum_ieee(&entry_vec[CHECK_START_OFFSET..]);

                // No need for check
                let dat_safe = unsafe {
                    Some(&mut *(entry_vec.as_ptr() as *mut DataEntryHeader))
                };
                dat_safe.unwrap().checksum = checksum;
                Ok(entry_vec)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
}
