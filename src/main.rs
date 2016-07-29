extern crate viola;

use viola::gap::GapBuffer;

fn main() {
    let buf = GapBuffer::with_gap(7);
    println!("{:?}", buf);
}
