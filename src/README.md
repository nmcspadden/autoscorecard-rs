# AutoScorecard

This very basic tool takes a (component) Apple package and generates an Inspec test that tests for the existence of every file in the payload.

Right now, there is no CLI interface so the path is hardcoded in the script.  You can edit this to be whatever you want, and then execute via `cargo run` inside the project directory to have it generate an Inspec profile. At the moment, you will need Rust installed for this to work.

# Inspec tests

The output of the tool currently creates an inspec profile (in the same directory as the code). This inspec profile has a basic inspec.yml (currently hardcoded to be authored by me, so something you'll have to edit in the code itself), and a single control that tests for the existence of every file in the payload.

In order to use the test, you will need to have Inspec installed.

To execute the test:
```
inspec exec <path to inspec profile folder>
```

This will test (on your local machine) for the existence of every file declared in a package payload, and give you the detailed results. See Inspec's own documentation for more features of how you can leverage this to produce reports/data in various formats (such as JSON).

# Current limitations

This is a very lightweight tool at this time, no CLI scaffolding has been built yet.

The tool currently only supports flat packages, and only supports the `payload` section of the package. Anything created by a postinstall script will be entirely left out of the test (or will cause a test to fail, if the postinstall script changes the path of a file that was in the payload).

It does not check for file mode, nor for directories - it takes everything declared in the package's BOM and naively just tests for them.

It also does not ignore/strip out known "empty" or non-existent files - such as the `._` files that are created by macOS when a file is copied to a filesystem that does not support extended attributes. They can be declared in a package's BOM, but they do not exist in the payload. This tool will currently test for them, and fail the test if they are not present. (Arguably, packages shouldn't contain these files in the first place, but that's a different discussion.)