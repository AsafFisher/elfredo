use elfredo::embeditor::run_embeditor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    id: u32,
}

fn main() {
    // This will print the embedded data if exists
    if let Err(err) = run_embeditor::<Person>() {
        panic!("{}", err);
    }
}
