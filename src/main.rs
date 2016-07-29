extern crate viola;

use viola::gap_buffer::GapBuffer;

fn main() {
    let mut buf = GapBuffer::from("Hello, world!");
    println!("{:?}", buf);
    buf.move_gap(0);
    println!("{:?}", buf);
    buf.insert("Good, world!");
    println!("{:?}", buf);
    buf.move_gap(4);
    println!("{:?}", buf);
    buf.insert("bye");
    println!("{:?}", buf);
    println!("{}", buf);
}
