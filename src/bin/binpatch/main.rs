use clap::{AppSettings, Clap};
use std::io::{Read, Write, Seek, SeekFrom};

use elfredo::{
    binpatch::{get_section, update_section},
    data_entry::DataEntryHeader
};
use std::path::{PathBuf, Path};

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    input: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    config: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}
fn main() {
    let opts: Opts = Opts::parse();
    let mut buffer = get_section(&opts.input, ".extended");
    //buffer.extend_from_slice(data_entry::EMBEDDED_MAGIC);
    let data = match DataEntryHeader::generate_entry(b"Hello".to_vec()){
        Ok(data) => data,
        Err(()) => panic!("Could not generate entry")
    };
    update_section(&Path::new(&opts.input), &data, ".extended");

    let buffer = get_section(&opts.input, ".extended");
    println!("{:?}", buffer);
}