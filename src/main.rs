extern crate viola;

use viola::gap_buffer::GapBuffer;

fn main() {
    let buf = GapBuffer::new();
    println!("{}", buf);
    println!("{:?}", buf);
}
