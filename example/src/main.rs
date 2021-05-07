use elfredo::get_embedded_data;
mod my_embeditor;
use my_embeditor::Person;

fn main() -> Result<(), failure::Error> {
    let person = get_embedded_data::<Person>().unwrap();
    println!("{:?}", person);
    Ok(())
}
