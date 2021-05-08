#![feature(const_size_of_val)]
#[macro_use]
extern crate failure;

pub mod binpatch;
pub mod data_entry;

#[no_mangle]
pub fn get_embedded_data() -> Result<&'static [u8], data_entry::ElfReadoError> {
    data_entry::EXTENDED_DATA.get_data()
}
