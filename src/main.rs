extern crate viola;

use viola::gap_buffer::GapBuffer;

fn main() {
    let mut buf = GapBuffer::new();
    for _ in 0..11 {
        buf.insert("Hello, world!");
        println!("{:?}", buf);
    }
}
