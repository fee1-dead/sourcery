use std::backtrace::Backtrace;
use std::cell::Cell;
use std::env::current_dir;
use std::fs::read_to_string;
use std::panic::{UnwindSafe, catch_unwind};
use std::path::Path;

use libtest_mimic::{Failed, Trial};
use walkdir::WalkDir;

static IDEMPOTENCE: Variant<String> = Variant {
    name: "Idempotence",
    runner: |content| {
        let mut file = sourcery::parse(&content);
        let mut content2 = String::new();
        sourcery::passes::format_with_style_guide(&mut file);
        sourcery::Print::print(&file, &mut content2);
        let content = content.trim_end();
        let content2 = content2.trim_end();
        if content != content2 {
            panic!("different content\norig   : {content:?}\nprinted: {content2:?}");
        }
    },
};

static FMT: Variant<(String, String)> = Variant {
    name: "Format",
    runner: |(content1, minified)| {
        let mut file = sourcery::parse(&content1);
        let mut content2 = String::new();
        sourcery::passes::format_with_style_guide(&mut file);
        sourcery::Print::print(&file, &mut content2);
        let minified = minified.trim_end();
        let content2 = content2.trim_end();
        if minified != content2 {
            panic!("different content\nexpected : {minified:?}\nformatted: {content2:?}");
        }
    },
};


pub struct Variant<T> {
    name: &'static str,
    runner: fn(T),
}

impl<T: UnwindSafe + Send> Variant<T> {
    pub fn make_trial(&'static self, path: String, content: T) -> Trial {
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

// TODO dedup
pub fn add_tests(tests: &mut Vec<Trial>) -> color_eyre::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let current_dir = current_dir()?;
    let walk = WalkDir::new(manifest_dir.join("tests/style"));

    for dir in walk {
        let ent = dir?;
        if !ent.file_type().is_file() {
            continue;
        }

        let path = ent.into_path();
        let name = path.strip_prefix(&current_dir)?.display().to_string();
        let content = read_to_string(&path)?;
        if name.ends_with(".fmt.rs") {
            tests.push(IDEMPOTENCE.make_trial(name, content))
        } else {
            let p = path.with_extension("fmt.rs");
            if !p.exists() {
                panic!("need {}", p.display());
            }
            let expected = read_to_string(&p)?;
            tests.push(FMT.make_trial(name, (content, expected)))
        }
    }

    Ok(())
}

use std::process::ExitCode;

use libtest_mimic::Arguments;

fn main() -> color_eyre::Result<ExitCode> {
    let args = Arguments::from_args();

    let mut tests = vec![];

    add_tests(&mut tests)?;
    Ok(libtest_mimic::run(&args, tests).exit_code())
}
