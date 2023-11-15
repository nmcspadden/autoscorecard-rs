// This was written for inspec 5.21.29+

/*
    inspec init profile <profile name>
    inspec check <path to folder>
    1. Edit inspec.yml
    2. Extract BOM from .pkg file
    3. Create new control with files and modes 

*/

fn extract_contents() {
    // extract or examine payload from archive
}

fn extract_pkg_payload() {
    // extract payload from pkg file
}

fn extract_zip_payload() {
    // extract payload from zip file
}

fn extract_dmg_payload() {
    // extract payload from dmg file
}

fn extract_rpm_payload() {
    // extract payload from rpm file
}

fn create_inspec_profile() {
    // create the inspec init here
}

fn edit_inspec_yml() {
    // edit the inspec.yml here
}

fn edit_inspec_readme() {
    // edit the inspec readme here
}

fn create_control() {
    // create the control for the payload files here
}

fn check_control() {
    // Run inspec check on finished control to validate
}
fn main() {
    println!("Hello, world!");
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