#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestObj {
    name: &'static str,
    age: u8,
}

#[cfg(test)]
mod tests {
    use elfredo::{
        data_entry::DataEntryHeader,
        embeditor::{get_section, update_section},
    };

    use crate::TestObj;
    use bytesize;
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::ffi::OsStr;
    use std::fmt::Debug;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use tempfile::NamedTempFile;

    pub const TEST_STRING: &[u8] = b"\"Oh, how I wish I could shut up like a telescope! \
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

    fn generate_tmp_file_clone(file_to_clone: &Path) -> NamedTempFile {
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
    fn test_patch_u8_vec() {
        test_patching_generic(&TEST_STRING.to_vec(), |expected, result| {
            assert_eq!(expected, result);
        });
    }

    #[test]
    fn test_patch_stress_u8_vec() {
        // ~110 KIB was the max objcopy could handle...
        // TODO: check why
        let huge_blob = b"A".repeat(bytesize::KIB as usize * 110);
        test_patching_generic(&huge_blob.to_vec(), |expected, result| {
            assert_eq!(expected, result);
        });
    }

    pub const TEST_OBJ: TestObj = TestObj {
        name: "Moshe",
        age: 3,
    };

    #[test]
    fn test_patch_object() {
        test_patching_generic(&TEST_OBJ, |expected, result| {
            assert_eq!(
                format!("{:?}", expected),
                String::from_utf8(result.to_vec()).unwrap()
            );
        });
    }

    #[test]
    fn test_dump_json() {
        test_dump_generic(&TEST_OBJ, &|json_result| {
            assert_eq!(
                json_result,
                serde_json::to_string_pretty(&TEST_OBJ).unwrap()
            )
        })
    }

    fn prepare_test_requirements<T: Debug + Serialize>(test_data: &T) -> (NamedTempFile, Vec<u8>) {
        // Our embedded sample data
        let data = DataEntryHeader::generate_entry(test_data).expect("Could not generate entry");

        // Gets our test elf template
        let elf_path = get_test_elf_path();
        assert!(elf_path.exists(), "{:?} does not exists", elf_path);

        // Generate a tmp file from our elf template.
        let tmp_file = generate_tmp_file_clone(&elf_path);
        (tmp_file, data)
    }
    fn test_patching_generic<T: Debug + Serialize>(
        test_data: &T,
        pass_condition: fn(&T, &Vec<u8>),
    ) {
        // Prepare the test environment
        let (tmp_elf_file, data) = prepare_test_requirements(test_data);

        // Patch the elf section
        update_section(tmp_elf_file.path(), &data, ".extended");

        let output = std::process::Command::new(tmp_elf_file.path())
            .arg(std::any::type_name::<T>().split("::").last().unwrap())
            .output()
            .expect("failed to execute process");
        pass_condition(test_data, &output.stdout)
    }
    fn test_dump_generic<'de, T: Debug + Serialize + Deserialize<'de>>(
        test_data: &T,
        pass_condition: &dyn Fn(&str),
    ) {
        // Prepare the test environment
        let (tmp_elf_file, data) = prepare_test_requirements(test_data);
        // Patch the elf section
        update_section(tmp_elf_file.path(), &data, ".extended");
        let dump = get_section(tmp_elf_file.path().to_str().unwrap(), ".extended");
        pass_condition(
            serde_json::to_string_pretty::<T>(
                &DataEntryHeader::ptr_to_data(dump.as_slice()).unwrap(),
            )
            .unwrap()
            .as_str(),
        )
    }
}
