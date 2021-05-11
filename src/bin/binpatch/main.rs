use clap::{AppSettings, Clap};

use elfredo::{binpatch::update_section, data_entry::DataEntryHeader};
use std::path::Path;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    elf_target: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    file_to_embed: String,
}

fn main() -> Result<(), failure::Error> {
    let opts: Opts = Opts::parse();
    let bytes = std::fs::read(&opts.file_to_embed)?;
    let data = DataEntryHeader::generate_entry(bytes).expect("Could not generate entry");
    update_section(&Path::new(&opts.elf_target), &data, ".extended");
    Ok(())
}
