extern crate viola;

use viola::gap::GapBuffer;

fn main() {
    let mut buf = GapBuffer::new();
    println!("{:?}", buf);
    buf.replace((0..0).into(), &[2]);
    println!("{:?}", buf);
    buf.replace((0..0).into(), &[1]);
    println!("{:?}", buf);
    buf.replace((0..2).into(), &[]);
    println!("{:?}", buf);
    buf.replace((0..0).into(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("{:?}", buf);
    buf.replace((4..8).into(), &[11, 12]);
    println!("{:?}", buf);
    buf.replace((6..6).into(), &[0, 0, 0, 0, 0, 0]);
    println!("{:?}", buf);
}
