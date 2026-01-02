use sourcery::Print;

fn main() {
    let src = " const X: i32 = (1);";
    let f = sourcery::parse(src);
    println!("{f:#?}");
    let mut s = String::new();
    f.print(&mut s);
    println!("{s}");
}
