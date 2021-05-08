#![feature(const_size_of_val)]
pub mod binpatch;
pub mod data_entry;

#[no_mangle]
pub fn get_embedded_data() -> Result<&'static [u8], ()> {
    data_entry::EXTENDED_DATA.get_data()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("Hello");
        //get_entry().unwrap();
    }
}
