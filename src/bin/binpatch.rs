use clap::{AppSettings, Clap};
use tempfile::NamedTempFile;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;

#[path = "../data_entry.rs"] // Here
mod data_entry;

const OBJCOPY_PATH: &str = "/usr/bin/objcopy";
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
fn get_section(elf_file: &String, section_name: &str) -> Vec<u8>{
    let tmp_file_path = String::from(NamedTempFile::new()
        .expect("Unable to create tmp file")
        .into_temp_path().to_str()
        .expect("Path unicode error"));
    let output = std::process::Command::new(OBJCOPY_PATH)
        .arg(elf_file)
        .arg("--dump-section")
        .arg(
            format!(
                "{}={}", section_name, tmp_file_path
            )).output().expect("Objcopy failed");
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    std::fs::read(tmp_file_path).expect("Could not read dumped section")
}
fn update_section(elf_file: &String, section_data: &Vec<u8>, section_name: &str){
    let mut tmp_file = NamedTempFile::new().expect("Could not create tmp file");
    tmp_file.write_all(section_data.as_slice()).expect("Could not write section");

    let tmp_file_path = String::from(
        tmp_file.path().to_str().expect("Path unicode error"));
    println!("{:?}", tmp_file_path);

    let output = std::process::Command::new(OBJCOPY_PATH)
        .arg(elf_file)
        .arg("--update-section")
        .arg(
            format!(
                "{}={}", section_name, tmp_file_path
            )).output().expect("Objcopy failed");
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}
fn main() {
    let opts: Opts = Opts::parse();
    let mut buffer = get_section(&opts.input, ".data");
    buffer.extend_from_slice(data_entry::EMBEDDED_MAGIC);
    update_section(&opts.input, &buffer, ".data");


    let buffer = get_section(&opts.input, ".data");
    println!("{:?}", buffer);
}