use anyhow::Result;
use apple_bom::{BomPath, ParsedBom};
use apple_flat_package::{ComponentPackageReader, PkgReader};
use std::fs::File;
use std::path::{Path, PathBuf};

/*

    APPLE .PKG FUNCTIONS

*/
pub fn extract_pkg_payload(source: &Path) -> Result<Vec<PathBuf>> {
    // extract payload from pkg file
    let package: ComponentPackageReader = get_component_pkg_from_path(source)?;

    // Now that we have a package file validated, get the root install location
    let root_dir: PathBuf = get_install_location_from_component_pkg(&package);
    // We can safely unwrap because the package must have a BOM
    let bom_data: &[u8] = package.bom().unwrap();
    let bom: ParsedBom<'_> = ParsedBom::parse(&bom_data)?;
    let paths: Vec<PathBuf> = get_pkg_payload_paths(bom, &root_dir)?;
    Ok(paths)
}

fn get_pkg_payload_bompaths(bom: ParsedBom) -> Result<Vec<BomPath>> {
    let paths = bom.paths()?;
    println!("Number of files: {}", paths.len());
    Ok(paths)
}

fn get_pkg_payload_paths(bom: ParsedBom, install_root: &PathBuf) -> Result<Vec<PathBuf>> {
    let bompaths = get_pkg_payload_bompaths(bom)?;
    let mut paths: Vec<PathBuf> = Vec::new();
    for bompath in bompaths.iter() {
        let mut joined_path: PathBuf = PathBuf::from(install_root);
        let mut bom_path_pb: PathBuf = PathBuf::from(bompath.path());
        // All bom paths start with ".", which needs to be removed
        bom_path_pb = bom_path_pb.strip_prefix(".")?.to_path_buf();

        joined_path.push(bom_path_pb);
        println!("{}", joined_path.display());
        paths.push(joined_path);
    }
    println!("Assembled all paths");
    Ok(paths)
}

fn print_paths_short(bom_paths: Vec<BomPath>) {
    for path in bom_paths {
        if let Some(link) = path.link_name() {
            println!("{} {} -> {}", path.symbolic_mode(), path.path(), link);
            // println!("{} {} -> {}", path.file_mode(), path.path(), link);
        } else {
            println!("{} {}", path.symbolic_mode(), path.path());
            // println!("{} {}", path.file_mode(), path.path());
        }
    }
}

fn get_component_pkg_from_path(
    source: &Path,
) -> Result<apple_flat_package::ComponentPackageReader> {
    // for now, this only accepts a single component package
    // we'll have to figure out distribution packages later
    let mut reader: PkgReader<File> = PkgReader::new(File::open(source)?)?;
    // let packages: Vec<apple_flat_package::ComponentPackageReader> = reader.component_packages()?;
    // println!("Number of components: {}", packages.len());
    // let package: Option<ComponentPackageReader> = reader.root_component()?;
    // let resolve: Option<ComponentPackageReader> = reader.resolve_component("Payload")?;
    // if resolve.is_some() {
    //     let component: ComponentPackageReader = resolve.unwrap();
    //     println!("Resolved payload: {:?}", component.package_info());
    // }
    let package_flavor: apple_flat_package::PkgFlavor = reader.flavor();
    println!("Flavor: {:?}", package_flavor);
    let package: Option<ComponentPackageReader> = reader.root_component()?;
    let component_result = match package {
        None => panic!("Not a component package!"),
        Some(x) => x,
    };
    // let package_info: Option<&apple_flat_package::PackageInfo> = temp_pkg.package_info();
    // println!("Package info: {:?}", package_info.unwrap());
    // let pkg_info = package_info.unwrap();
    // let install_location = pkg_info.install_location.as_ref().unwrap();
    // println!("***Install location: {:?}", install_location);
    // let payload: &Option<apple_flat_package::package_info::Payload> = &package_info.unwrap().payload;
    // println!("Payload: {:?}", payload.clone().unwrap());
    Ok(component_result)
}

fn get_install_location_from_component_pkg(pkg: &ComponentPackageReader) -> PathBuf {
    // Any properly formed Apple pkg should have a PkgInfo file; if not, we bail
    let package_info = pkg
        .package_info()
        .expect("The package is malformed and has no PkgInfo!");
    println!("Package info: {:?}", package_info);

    // let install_location = package_info.install_location.as_ref().expect("PackageInfos must have an install location");
    let install_location = match package_info.install_location.as_ref() {
        None => "/",
        Some(x) => x,
    };
    println!("Install location: {:?}", install_location);
    // let payload: &Option<apple_flat_package::package_info::Payload> = &package_info.unwrap().payload;
    // println!("Payload: {:?}", payload.clone().unwrap());

    return PathBuf::from(install_location);
}
