use sourcery::Print;
use sourcery::parse::Precedence;

fn main() {
    assert!(Precedence::MIN < Precedence::Or);
    let src = " const X: i32 = (1);";
    let f = sourcery::parse(src);
    println!("{f:#?}");
    let mut s = String::new();
    f.print(&mut s);
    println!("{s}");
}
