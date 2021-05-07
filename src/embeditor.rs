use crate::data_entry::DataEntryHeader;
use clap::{AppSettings, Clap};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

const OBJCOPY_PATH: &str = "/usr/bin/objcopy";

pub fn get_section(elf_file: &str, section_name: &str) -> Vec<u8> {
    let tmp_file_path = String::from(
        NamedTempFile::new()
            .expect("Unable to create tmp file")
            .into_temp_path()
            .to_str()
            .expect("Path unicode error"),
    );
    let output = std::process::Command::new(OBJCOPY_PATH)
        .arg(elf_file)
        .arg("--dump-section")
        .arg(format!("{}={}", section_name, tmp_file_path))
        .output()
        .expect("Objcopy failed");
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    std::fs::read(tmp_file_path).expect("Could not read dumped section")
}
pub fn update_section(elf_file: &Path, section_data: &[u8], section_name: &str) {
    let mut tmp_file = NamedTempFile::new().expect("Could not create tmp file");
    tmp_file
        .write_all(section_data)
        .expect("Could not write section");

    let tmp_file_path = String::from(tmp_file.path().to_str().expect("Path unicode error"));
    println!("{:?}", tmp_file_path);

    let output = std::process::Command::new(OBJCOPY_PATH)
        .arg(elf_file)
        .arg("--update-section")
        .arg(format!("{}={}", section_name, tmp_file_path))
        .output()
        .expect("Objcopy failed");
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Asaf F. <asaffisher.dev@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    elf_target: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    json_file: String,
}

pub fn run_embeditor<T: Serialize + DeserializeOwned>() -> Result<(), failure::Error> {
    let opts: Opts = Opts::parse();
    let person: T = serde_json::from_reader(std::fs::File::open(opts.json_file)?).expect("Founnd");
    let data = DataEntryHeader::generate_entry(person).expect("Could not generate entry");
    update_section(&Path::new(&opts.elf_target), &data, ".extended");
    Ok(())
}
