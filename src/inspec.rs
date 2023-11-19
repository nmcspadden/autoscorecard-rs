use serde::Deserialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

/*
    INSPEC FUNCTIONS
    This was written for inspec 5.21.29+
*/

pub fn create_inspec_profile(name: &OsStr) -> PathBuf {
    // create the inspec init here
    // We know for sure this path is safe to unwrap because we validated it already on being passed in
    let output_path = format!("profiles/{}", name.to_str().unwrap());
    println!("Creating inspec profile at: {}", output_path);
    let output: std::process::Output = Command::new("/opt/inspec/bin/inspec")
        .args(["init", "profile", &output_path, "--overwrite"])
        .output()
        .expect("Failed to execute inspec init");

    if output.status.success() {
        println!("Successfully created inspec profile");
    } else {
        println!("Failed to create inspec profile: {}", output.status);
        println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    // Return the path to the created profile
    let canon = std::fs::canonicalize(&output_path).unwrap();
    return PathBuf::from(canon);
}

pub fn edit_inspec_yml(profiles_path: &PathBuf) {
    // edit the inspec.yml here
    let mut yml_path = PathBuf::from(profiles_path);
    yml_path.push("inspec.yml");
    // TODO: do this with serde
}

pub fn edit_inspec_readme() {
    // edit the inspec readme here
}

pub fn create_control() {
    // create the control for the payload files here
}

pub fn check_control() {
    // Run inspec check on finished control to validate
}
