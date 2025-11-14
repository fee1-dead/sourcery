use sourcery::passes::{Minify, Pass};
use sourcery::{Print, parse};

fn main() {
    let src = " /* w */ mod foo {
        mod barrr ; // a
    }";
    let mut f = parse(src);
    let mut s = String::new();
    Minify.visit_file(&mut f);

    f.print(&mut s);
    println!("{s}");
}
