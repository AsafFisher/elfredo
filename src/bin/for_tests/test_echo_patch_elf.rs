use elfredo::get_embedded_data;

fn main() {
    print!("{}", String::from_utf8(get_embedded_data().unwrap().to_vec()).unwrap());
}
