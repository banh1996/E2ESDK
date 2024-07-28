#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;
    //use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_c_wrapper() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        // fs::copy("include/e2esdk.h", out_dir.join("e2esdk.h")).expect("Failed to copy header file");
        // fs::copy("target/debug/e2esdk.dll", out_dir.join("e2esdk.dll")).expect("Failed to copy header file");

        let status = Command::new("gcc")
            .args(&[
                "-o", &format!("{}/c_wrapper_test", out_dir.display()),
                "tests/c_wrapper_test.c",
                "-L", "target/release",
                "-le2esdk",
                "-Iinclude",
            ])
            .status()
            .expect("Failed to compile C test");

        assert!(status.success());

        let status = Command::new(format!("{}/c_wrapper_test", out_dir.display()))
            .status()
            .expect("Failed to execute test");

        assert!(status.success());
    }
}