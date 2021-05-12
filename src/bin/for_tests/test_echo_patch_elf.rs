use clap::Clap;
use elfredo::get_embedded_data;

#[path = "../../../tests/test_features.rs"]
mod test_features;

use test_features::TestObj;

#[derive(Clap)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    test_name: String,
}

fn test_vector_u8() -> Result<(), failure::Error> {
    print!(
        "{}",
        String::from_utf8(get_embedded_data::<Vec<u8>>()?).unwrap()
    );
    Ok(())
}

fn test_object() -> Result<(), failure::Error> {
    print!("{:?}", get_embedded_data::<TestObj>().unwrap());
    Ok(())
}

fn main() -> Result<(), failure::Error> {
    let opts: Opts = Opts::parse();
    match opts.test_name.as_str() {
        "Vec<u8>" => test_vector_u8()?,
        "TestObj" => test_object()?,
        _ => panic!("Error"),
    };
    Ok(())
}
