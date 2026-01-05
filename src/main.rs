use sourcery::Print;
use sourcery::parse::Precedence;

fn main() {
    assert!(Precedence::MIN < Precedence::Or);
    let src = " a.0.b.1.1.c";
    let f = sourcery::parse_to_tokenstream(src);
    println!("{f:?}");
    let mut s = String::new();
    f.print(&mut s);
    println!("{s}");
}
