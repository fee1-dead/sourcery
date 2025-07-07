use sourcery::{Print, parse};

fn main() {
    let src = " /* w */ mod foo {
        mod barrr ; // a
    }";
    let f = parse(src);
    println!("{f:#?}");
    let mut s = String::new();
    f.print(&mut s);
    assert_eq!(src, s);
}
