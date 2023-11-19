// For development purposes
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::path::Path;

// This was written for inspec 5.21.29+
use autoscorecard_rs::{extract_contents, create_inspec_profile, edit_inspec_yml};

fn main() {
    let input = "/Users/nmcspadden/Downloads/AutoPkg-only-3.0.0RC2.pkg";
    let source = Path::new(input);
    println!("Loading up pkg file: {}", source.display());

    extract_contents(source);
    let profiles_path = create_inspec_profile(source.file_stem().unwrap());
    edit_inspec_yml(&profiles_path);
    /*
    1. extract_contents() - Determine type and extract archive/obtain BOM
    2. extract_<type>_payload() - extract archive/obtain BOM
    3. create_inspec_profile() - init new inspec profile
    4. edit_inspec_yml() - edit yml with package data
    5. edit_inspec_readme() - edit readme with package data
    6. create_control() - create control with payload of files and modes
    7. check_control() - run inspec check on control
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_contents() {
        // test extract_contents()
    }

    #[test]
    fn test_extract_pkg_payload() {
        // test extract_pkg_payload()
    }

    #[test]
    fn test_extract_zip_payload() {
        // test extract_zip_payload()
    }

    #[test]
    fn test_extract_dmg_payload() {
        // test extract_dmg_payload()
    }

    #[test]
    fn test_extract_rpm_payload() {
        // test extract_rpm_payload()
    }

    #[test]
    fn test_create_inspec_profile() {
        // test create_inspec_profile()
    }

    #[test]
    fn test_edit_inspec_yml() {
        // test edit_inspec_yml()
    }

    #[test]
    fn test_edit_inspec_readme() {
        // test edit_inspec_readme()
    }

    #[test]
    fn test_create_control() {
        // test create_control()
    }

    #[test]
    fn test_check_control() {
        // test check_control()
    }
}
