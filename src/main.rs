use sourcery::{Print, parse_to_tokenstream};

fn main() {
    let src = " 'a";
    let f = parse_to_tokenstream(src);
    println!("{f:?}");
    let mut s = String::new();
    f.print(&mut s);
    println!("{s}");
}
