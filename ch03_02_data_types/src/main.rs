use std::io;

fn main() {
    let x: u32 = 3;

    println!("x: u32 = {x}");

    let y: f32 = 3.0;
    println!("y: f32 = {y}");

    let z: bool = true;
    println!("z: bool = {z}");

    let ch: char = 'Z';
    println!("ch: char = {ch}");

    let ch = 'z';
    println!("ch: char = {ch}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: (i32, f64, u8) = ({x} {y} {z})");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
