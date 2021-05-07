#![feature(const_size_of_val)]
pub mod binpatch;
pub mod data_entry;

pub fn get_data() -> Result<&'static [u8], ()> {
    data_entry::extended_data.get_data()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("Hello");
        //get_entry().unwrap();
    }
}
