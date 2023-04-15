fn multiply(x: i16, y: i16) -> i16 { x * y }

pub fn main() {
    let x: i16 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}