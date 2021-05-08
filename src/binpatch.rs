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
