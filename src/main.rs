extern crate viola;

use viola::byte_range::ByteRange;
use viola::gap::GapString;

fn main() {
    let mut buf = GapString::new();
    println!("{:?}", buf);
    buf.replace(ByteRange::from(0..0), "Good, ");
    println!("{:?}", buf);
    buf.replace(ByteRange::from(6..6), "world!");
    println!("{:?}", buf);
    buf.replace(ByteRange::from(4..4), "bye");
    println!("{:?}", buf);
    buf.replace(ByteRange::from(0..7), "hello");
    println!("{:?}", buf);
    buf.replace(ByteRange::from(0..1), "H");
    println!("{:?}", buf);

    println!("{}", buf);
}
