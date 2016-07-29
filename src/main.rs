extern crate viola;

use viola::gap_buffer::GapBuffer;

fn main() {
    let buf = GapBuffer::from("Hello, world!");
    println!("{}", buf);
    println!("{:?}", buf);
}
