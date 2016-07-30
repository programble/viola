extern crate viola;

use viola::gap::GapString;

fn main() {
    let mut buf = GapString::new();
    println!("{:?}", buf);
    buf.splice(0..0, "Good, ");
    println!("{:?}", buf);
    buf.splice(6..6, "world!");
    println!("{:?}", buf);
    buf.splice(4..4, "bye");
    println!("{:?}", buf);
    buf.splice(0..7, "hello");
    println!("{:?}", buf);
    buf.splice(0..1, "H");
    println!("{:?}", buf);

    println!("{}", buf);
}
