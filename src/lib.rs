// For development purposes
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// use {
//     anyhow::{Error, Result},
//     apple_bom::{BomPath, ParsedBom},
//     apple_flat_package::{ComponentPackageReader, PkgReader},
//     std::{
//         ffi::OsStr,
//         fmt::Debug,
//         fs::File,
//         io::{BufRead, Cursor, Read, Seek, SeekFrom, Write},
//         path::{Path, PathBuf},
//         process::{Command, Stdio},
//         time::Duration,
//     },
// };

mod apple_pkg;

use std::path::Path;

use crate::apple_pkg::extract_pkg_payload;

pub fn extract_contents(source: &Path) {
    // extract or examine payload from archive
    let _ = extract_pkg_payload(source);
}

pub fn extract_zip_payload() {
    // extract payload from zip file
}

pub fn extract_dmg_payload() {
    // extract payload from dmg file
}

pub fn extract_rpm_payload() {
    // extract payload from rpm file
}
