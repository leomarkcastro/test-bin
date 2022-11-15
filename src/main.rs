#![deny(clippy::all)]

use test_lib::addition::add;
use test_lib::subtraction::sub;

fn main() {
    let a = 5;
    let b = 3;

    let sum = add(a, b);
    let diff = sub(a, b);

    println!("{} {}", sum, diff);
}
