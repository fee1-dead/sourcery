use std::process::ExitCode;

use libtest_mimic::Arguments;

mod pp;
mod minify;

fn main() -> color_eyre::Result<ExitCode> {
    let args = Arguments::from_args();

    let mut tests = vec![];

    pp::add_tests(&mut tests)?;
    minify::add_tests(&mut tests)?;

    Ok(libtest_mimic::run(&args, tests).exit_code())
}
