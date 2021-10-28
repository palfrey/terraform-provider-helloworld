use std::{env, process::Command};

#[test]
fn run_tf() {
    let path = env::current_dir().unwrap();
    let test_path = path.join("tf");
    env::set_current_dir(test_path).unwrap();
    let status = Command::new("../terraform").arg("plan").status().unwrap();
    assert!(status.success());
}