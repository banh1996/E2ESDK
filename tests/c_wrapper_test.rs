#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;
    use std::io::Write;
    use std::fs::{self, File};
    use std::path::{PathBuf, Path};

    fn setup_test_dir(test_dir: &str) -> PathBuf {
        let path = Path::new(test_dir);

        //println!("Create test_folder: {:?}", path.to_path_buf());
    
        if path.exists() {
            if let Err(e) = fs::remove_dir_all(path) {
                eprintln!("Failed to remove directory: {}", e);
            }
        }
    
        if let Err(e) = fs::create_dir(path) {
            eprintln!("Failed to create directory: {}", e);
        }
    
        path.to_path_buf()
    }

    fn create_test_file(test_dir: &str, name: &str, content: &[u8]) -> PathBuf {
        let path = Path::new(test_dir);
        let file_path = path.join(name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content).unwrap();
        file_path
    }

    fn cleanup_test_dir(test_dir: &str) {
        let path = Path::new(test_dir);
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    #[test]
    fn test_c_wrapper_basic_functions() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        //println!("running commands from dir {:?}", out_dir);
        let status = Command::new("gcc")
            .args(&[
                "-o", &format!("{}/c_wrapper_test", out_dir.display()),
                "tests/c_wrapper_test.c",
                "-L", "target/debug",
                "-le2esdk",
                "-Iinclude",
            ])
            .status()
            .expect("Failed to compile C test");

        assert!(status.success());

        let status = Command::new(format!("{}/c_wrapper_test", out_dir.display()))
            .args(&[
                &format!("{}", out_dir.display()),
                "0",
            ])
            .status()
            .expect("Failed to execute test");

        assert!(status.success());
    }

    #[test]
    fn test_c_wrapper_exsecure() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        //println!("running commands from dir {:?}", out_dir);
        let status = Command::new("gcc")
            .args(&[
                "-o", &format!("{}/c_wrapper_test", out_dir.display()),
                "tests/c_wrapper_test.c",
                "-L", "target/debug",
                "-le2esdk",
                "-Iinclude",
            ])
            .status()
            .expect("Failed to compile C test");

        assert!(status.success());

        //const PASSWORD: &str = "testpassword";
        const TEST_DIR: &str = "test_folder";
        let _ = setup_test_dir(&format!("{}/{}", out_dir.display(), TEST_DIR));
        let _ = create_test_file(&format!("{}/{}", out_dir.display(), TEST_DIR), "test1.txt", b"Hello world! Im robot1");
        let _ = create_test_file(&format!("{}/{}", out_dir.display(), TEST_DIR), "test2.txt", b"Hello world! Im robot2");

        let status = Command::new(format!("{}/c_wrapper_test", out_dir.display()))
            .args(&[
                &format!("{}", out_dir.display()),
                "1",
            ])
            .status()
            .expect("Failed to execute test");

        let _ = cleanup_test_dir(&format!("{}/{}", out_dir.display(), TEST_DIR));
        assert!(status.success());
    }
}