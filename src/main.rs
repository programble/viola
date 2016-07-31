extern crate viola;

use viola::gap::string::GapString;

fn main() {
    let mut buf = GapString::new();
    println!("{:?}", buf);
    buf.splice(.., "Good, ");
    println!("{:?}", buf);
    buf.splice(6.., "world!");
    println!("{:?}", buf);
    buf.splice(4..4, "bye");
    println!("{:?}", buf);
    buf.splice(..7, "hello");
    println!("{:?}", buf);
    buf.splice(..1, "H");
    println!("{:?}", buf);

    println!("{}", buf);
}
