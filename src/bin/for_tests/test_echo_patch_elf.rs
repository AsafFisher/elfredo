use elfredo::get_embedded_data;

fn main() -> Result<(), failure::Error>{
    print!(
        "{}",
        String::from_utf8(get_embedded_data::<Vec<u8>>()?.to_vec()).unwrap()
    );
    Ok(())
}
