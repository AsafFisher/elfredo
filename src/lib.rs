#![feature(const_size_of_val)]
use std::slice;
use std::convert::TryInto;

pub mod data_entry;
extern {
    static mut __data_start: u8;
    static mut _edata: u8;
}

pub fn get_data() -> &'static [u8]{
    unsafe {
        println!("{:p}", &_edata as *const u8);
        let hello = &_edata;
        let length = (hello as *const u8).offset_from(&__data_start as *const _).abs() as usize;
        let slice = slice::from_raw_parts(&__data_start, length);
        slice
    }
}
pub fn get_entry() -> Result<(), ()>{
    let data = get_data();
    for ch in data[8..].iter(){
        println!("{:#4x}", ch);
    }

    const magic_size: usize = std::mem::size_of_val(data_entry::EMBEDDED_MAGIC);
    println!("dd{:?}", data.len());
    if data.len() >= magic_size{
        for i in 0..data.len(){
            if data.len() >= i + magic_size{
                let magic:[u8; 4] = data[i..i + magic_size].try_into().unwrap();
                //println!("{:?}", magic);
            }else{
                break;
            }
        }

        Ok(())
    }else{
        Err(())
    }
}
// fn get_embedded_data() -> DataEntry{
//
// }

#[cfg(test)]
mod tests {
    use crate::{get_entry};

    #[test]
    fn it_works() {
        get_entry().unwrap();
    }
}
