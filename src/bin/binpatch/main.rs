use clap::{AppSettings, Clap};

use elfredo::{
    binpatch::{get_section, update_section},
    data_entry::DataEntryHeader,
};
use std::path::Path;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    input: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[allow(dead_code)]
    config: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    #[allow(dead_code)]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let data =
        DataEntryHeader::generate_entry(b"Hello".to_vec()).expect("Could not generate entry");
    update_section(&Path::new(&opts.input), &data, ".extended");

    let buffer = get_section(&opts.input, ".extended");
    println!("{:?}", buffer);
}
