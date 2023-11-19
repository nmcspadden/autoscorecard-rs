use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

/*
    INSPEC FUNCTIONS
    This was written for inspec 5.21.29+
*/

/*
name: fileChecker
title: Check for files
maintainer: nmcspadden
copyright: nmcspaddens
copyright_email: nmcspadden@gmail.com
license: Apache-2.0
summary: Check files inside a package
version: 0.1.0
supports:
  platform: darwin
*/

#[derive(Debug, Deserialize)]
struct InspecYml {
    name: String,
    title: String,
    maintainer: String,
    copyright: String,
    copyright_email: String,
    license: String,
    summary: String,
    version: String,
    supports: HashMap<String, String>,
}

pub const DARWIN_PLATFORM: &str = "darwin";
pub const WINDOWS_PLATFORM: &str = "windows";
pub const FEDORA_PLATFORM: &str = "fedora";

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

pub fn edit_inspec_yml(profiles_path: &PathBuf) -> Result<()>{
    // edit the inspec.yml here
    let mut yml_path = PathBuf::from(profiles_path);
    yml_path.push("inspec.yml");
    // TODO: do this with serde
    let yml: InspecYml = read_inspec_yml(&yml_path)?;
    println!("{:?}", yml);
    Ok(())
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

fn read_inspec_yml(yml_path: &Path) -> Result<InspecYml> {
    let mut file = File::open(yml_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let yml: InspecYml = serde_yaml::from_str(&contents)?;
    Ok(yml)
}
