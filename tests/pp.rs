//! Parse-Print tests ensure that something can be parsed and printed.

use std::backtrace::Backtrace;
use std::cell::Cell;
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
            thread_local! {
                static BACKTRACE: Cell<Option<Backtrace>> = const { Cell::new(None) };
            }

            std::panic::set_hook(Box::new(|_| {
                let trace = Backtrace::force_capture();
                BACKTRACE.with(move |b| b.set(Some(trace)));
            }));

            let content = fs::read_to_string(&path).map_err(Failed::from)?;
            let parsed = match catch_unwind(|| sourcery::parse(&content)) {
                Ok(parsed) => parsed,
                Err(msg) => {
                    let msg = msg
                        .downcast_ref::<String>()
                        .map(|x| &**x)
                        .or_else(|| msg.downcast_ref::<&str>().map(|x| *x))
                        .unwrap_or("unknown panic message");
                    let b = BACKTRACE.with(|b| b.take()).unwrap();
                    return Err(Failed::from(format!("panicked while parsing: {msg}\n{b}")));
                }
            };

            let mut content2 = String::new();
            sourcery::Print::print(&parsed, &mut content2);
            if content != content2 {
                return Err(Failed::from("different content"));
            }

            Ok(())
        }))
    }

    Ok(libtest_mimic::run(&args, tests).exit_code())
}
