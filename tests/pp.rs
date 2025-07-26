//! Parse-Print tests ensure that something can be parsed and printed.

use std::backtrace::Backtrace;
use std::cell::Cell;
use std::env::current_dir;
use std::fs::read_to_string;
use std::panic::catch_unwind;
use std::path::Path;

use libtest_mimic::{Failed, Trial};
use walkdir::WalkDir;

static VARIANTS: &[Variant] = &[
    Variant {
        name: "FilePrint",
        runner: |content| {
            let file = sourcery::parse(&content);
            let mut content2 = String::new();
            sourcery::Print::print(&file, &mut content2);
            if content != content2 {
                panic!("different content\norig   : {content:?}\nprinted: {content2:?}");
            }
        },
    },
    Variant {
        name: "TokenStreamPrint",
        runner: |content| {
            let file = sourcery::parse_to_tokenstream(&content);
            let mut content2 = String::new();
            sourcery::Print::print(&file, &mut content2);
            if content != content2 {
                panic!("different content\norig   : {content:?}\nprinted: {content2:?}");
            }
        },
    },
];

pub struct Variant {
    name: &'static str,
    runner: fn(String),
}

impl Variant {
    pub fn make_trial(&'static self, path: String, content: String) -> Trial {
        let runner = move || {
            thread_local! {
                static BACKTRACE: Cell<Option<Backtrace>> = const { Cell::new(None) };
            }

            std::panic::set_hook(Box::new(|_| {
                let trace = Backtrace::force_capture();
                BACKTRACE.with(move |b| b.set(Some(trace)));
            }));

            match catch_unwind(move || (self.runner)(content)) {
                Ok(()) => Ok(()),
                Err(msg) => {
                    let msg = msg
                        .downcast_ref::<String>()
                        .map(|x| &**x)
                        .or_else(|| msg.downcast_ref::<&str>().map(|x| *x))
                        .unwrap_or("unknown panic message");
                    let b = BACKTRACE.with(|b| b.take()).unwrap();
                    Err(Failed::from(format!(
                        "panicked while running test: {msg}\n{b}"
                    )))
                }
            }
        };

        Trial::test(format!("{:<18} {path}", self.name), runner)
    }
}

pub fn add_tests(tests: &mut Vec<Trial>) -> color_eyre::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let current_dir = current_dir()?;
    let walk = WalkDir::new(manifest_dir.join("tests/pp"));

    for dir in walk {
        let ent = dir?;
        if !ent.file_type().is_file() {
            continue;
        }
        let path = ent.into_path();
        let name = path.strip_prefix(&current_dir)?.display().to_string();
        let content = read_to_string(&path)?;
        tests.extend(
            VARIANTS
                .iter()
                .map(|v| v.make_trial(name.clone(), content.clone())),
        );
    }

    Ok(())
}
