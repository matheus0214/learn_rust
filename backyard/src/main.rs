use crate::garden::vegetables::Aspargues;

pub mod garden;

fn main() {
    let plant = Aspargues {};
    println!("I'm growing {plant:?}!");
}
