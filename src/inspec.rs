use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
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

#[derive(Debug, Deserialize, Serialize)]
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
    let mut yml: InspecYml = read_inspec_yml(&yml_path)?;
    println!("Before: {:?}", yml);
    yml.title = format!("{} Payload Profile", yml.name);
    yml.maintainer = "nmcspadden".to_string();
    yml.copyright = "nmcpadden".to_string();
    yml.copyright_email = "nmcspadden@meta.com".to_string();
    yml.summary = format!("Payload of {} package", yml.name);
    yml.supports.insert("platform".to_string(), "darwin".to_string());
    println!("After: {:?}", yml);
    write_yml_to_disk(&yml_path, &yml)?;
    Ok(())
}

pub fn edit_inspec_readme() {
    // edit the inspec readme here
}

pub fn create_control(title: &str, paths: &Vec<PathBuf>, profiles_path: &PathBuf) -> Result<()> {
    // create the control text
    let control_text = create_control_text(title, paths);
    let mut control_path = PathBuf::from(profiles_path);
    control_path.push(format!("controls/{}.rb", title));
    write_inspec_test_to_disk(&control_path, &control_text)?;
    Ok(())
}

fn create_control_text(title: &str, paths: &Vec<PathBuf>) -> String {
    // create the control for the payload files here
    let title_str = format!("File Existence for {}", title);
    let title_block = create_title_block(&title_str);
    let mut file_string = String::new();
    file_string.push_str(&title_block);
    file_string.push_str("\n");
    let control_block = create_control_slug(title);
    file_string.push_str(&control_block);
    // file_string.push_str("\n");
    let files_block = create_describe_file_group_block(paths);
    file_string.push_str(&files_block);
    let ends = create_control_ends();
    file_string.push_str(&ends);

    return file_string
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

fn write_yml_to_disk(yml_path: &Path, yml: &InspecYml) -> Result<()> {
    let mut file = File::create(yml_path)?;
    let yml_string = serde_yaml::to_string(&yml)?;
    file.write_all(yml_string.as_bytes())?;
    Ok(())
}

fn write_inspec_test_to_disk(inspec_path: &Path, contents: &str) -> Result<()> {
    // write the inspec test to disk
    let mut file = File::create(inspec_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

/* Specific Inspec tools */
fn create_describe_file_exist_block(path: &Path) -> String {
    // The given file should exist
    return format!(
        r#"
describe file("{}") do
    it {{ should exist }}
end
    "#,
        path.display()
    );
}

fn create_describe_file_is_dir_block(path: &Path) -> String {
    // The given file should exist
    return format!(
        r#"
    describe file("{}") do
        it {{ should be_directory }}
    end
"#,
        path.display()
    );
}

fn create_describe_file_group_block(paths: &Vec<PathBuf>) -> String {
    // The given file should exist
    return format!(
        r#"
    files = [
{}
    ]
    files.each do |file_path|
        describe file(file_path) do
            it {{ should exist }}
        end
    end
"#,
        create_file_groups(paths)
    );
}

fn create_file_groups(paths: &Vec<PathBuf>) -> String {
    let mut file_groups: Vec<String> = Vec::new();
    for path in paths {
        file_groups.push(format!("\t\t'{}',", path.display()));
    }
    return file_groups.join("\n")
}

fn create_title_block(title: &str) -> String {
    // The given file should exist
    return format!(
        r#"
title "{}"
"#,
        title
    );
}

fn create_control_slug(title: &str) -> String {
    // only the intro of the control block here
    return format!(
        r#"
control "Informational test: {}" do
    impact 0.0
    title '{}'
    desc 'Check for existence of all files in a package'
"#,
        title,
        title,
    );
}

fn create_control_ends() -> String {
    // only the intro of the control block here
    return format!(
        r#"
end
"#,
    );
}
