use elfredo::get_data;
fn main() {
    print!("{}", String::from_utf8(get_data().unwrap().to_vec()).unwrap());
}
