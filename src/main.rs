extern crate viola;

use viola::gap::GapBuffer;

fn main() {
    let mut buf = GapBuffer::with_gap(5);
    println!("{:?}", buf);
    buf.insert(&[1, 2, 3]);
    println!("{:?}", buf);
    buf.insert(&[4, 5, 6]);
    println!("{:?}", buf);
}
