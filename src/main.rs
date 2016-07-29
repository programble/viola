extern crate viola;

use viola::gap::GapBuffer;

fn main() {
    let mut buf = GapBuffer::new();
    println!("{:?}", buf);
    buf.insert(&[4, 5, 6]);
    println!("{:?}", buf);
    buf.move_gap(0);
    println!("{:?}", buf);
    buf.insert(&[1, 2]);
    println!("{:?}", buf);
    buf.move_gap(5);
    println!("{:?}", buf);
    buf.insert(&[7, 8, 9]);
    println!("{:?}", buf);
    buf.move_gap(2);
    println!("{:?}", buf);
    buf.insert(&[3]);
    println!("{:?}", buf);
}
