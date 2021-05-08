use elfredo::{binpatch::update_section, data_entry::DataEntryHeader};

use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

const TEST_STRING: &[u8] = b"\"Oh, how I wish I could shut up like a telescope! \
I think I could, if only I knew how to begin.\
\" For, you see, so many out-of-the-way things had happened lately, \
that Alice had begun to think that very few things indeed were really impossible.\
 -Chapter 1, Down the Rabbit-Hole";

// Get absolute path to the "target" directory ("build" dir)
fn get_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}

fn get_test_elf_path() -> PathBuf {
    let mut elf_path = get_target_dir();
    elf_path.push(concat!("debug", "/", "test_echo_patch_elf"));
    elf_path
}

fn generate_tmp_file_clone(file_to_clone: &PathBuf) -> NamedTempFile {
    let file_buff = std::fs::read(file_to_clone).expect("Could not read elf");
    let mut tmp_file = NamedTempFile::new().expect("Could not create tmp file");

    // Copy file permissions.
    let file_to_clone_perms = std::fs::metadata(file_to_clone).unwrap().permissions();
    std::fs::set_permissions(tmp_file.path(), file_to_clone_perms).unwrap();

    tmp_file
        .write_all(file_buff.as_slice())
        .expect("Could not write section");
    tmp_file
}

#[test]
fn test_patching() {
    // Our embedded sample data
    let data =
        DataEntryHeader::generate_entry(TEST_STRING.to_vec()).expect("Could not generate entry");

    // Gets our test elf template
    let elf_path = get_test_elf_path();
    assert!(elf_path.exists(), "{:?} does not exists", elf_path);

    // Generate a tmp file from our elf template.
    let tmp_file = generate_tmp_file_clone(&elf_path);

    // Patch the elf section
    update_section(tmp_file.path(), &data, ".extended");

    let output = std::process::Command::new(tmp_file.path())
        .output()
        .expect("failed to execute process");
    assert_eq!(TEST_STRING, output.stdout);
}
