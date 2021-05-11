//! # Elfredo - Dynamic Resource Section
//! This module allows you to embed a resource into an elf section and change its content after
//! the binary has already been compiled. This is useful to change in-code configurations without
//! compiling the whole binary again. Simply apply `binpatch --raw new_data elf_file` to
//!
//! # Examples
//! app.rs
//! ```no_run
//! /// app.rs
//! use elfredo::get_embedded_data;
//! print!(
//!        "{}",
//!        String::from_utf8(get_embedded_data().unwrap().to_vec()).unwrap()
//! );
//! ```
//!
//! Roadmap
//! - [x] Make a simple binary patching ability.
//! - [ ] Make binary patching possible for generic types.
#![feature(const_size_of_val)]
#[macro_use]
extern crate failure;

pub mod binpatch;
pub mod data_entry;

#[no_mangle]
pub fn get_embedded_data() -> Result<&'static [u8], data_entry::ElfReadoError> {
    data_entry::EXTENDED_DATA.get_data()
}
