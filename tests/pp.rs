//! Parse-Print tests ensure that something can be parsed and printed.

use std::fs::{self, read_dir};
use std::panic::catch_unwind;
use std::path::Path;
use std::process::ExitCode;

use libtest_mimic::{Arguments, Failed, Trial};

fn main() -> color_eyre::Result<ExitCode> {
    let args = Arguments::from_args();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let read = read_dir(manifest_dir.join("tests/pp"))?;

    let mut tests = vec![];

    for dir in read {
        let ent = dir?;
        if !ent.file_type()?.is_file() {
            // TODO support recursive
            continue;
        }
        let file_name = ent.file_name();
        let s = file_name.to_string_lossy();
        let path = ent.path();
        tests.push(Trial::test(s.clone(), move || {
            let content = fs::read_to_string(&path).map_err(Failed::from)?;
            let Ok(parsed) = catch_unwind(|| sourcery::parse(&content)) else {
                return Err(Failed::from("panicked while parsing"))
            };
            let mut content2 = String::new();
            sourcery::Print::print(&parsed, &mut content2);
            if content != content2 {
                return Err(Failed::from("different content"))
            }
            
            Ok(())
        }))
    }

    Ok(libtest_mimic::run(&args, tests).exit_code())
}