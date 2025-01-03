use std::io;

fn main() {
    //floating point types
    let x = 2.0; //f64, double precision
    
    let y: f32 = 3.0; //f32, single precision

    //Integer types
    let a: u8 = 255;

    let b: u16 = 65535;

    let c: u32 = 2^32 - 1;

    //Boolean
    let t = true;
    let f: bool = false;

    //Characters - single quotes, single character. 4 bytes in size and can store more than just
    //ascii.
    let z = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    //Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (m, l, n) = tup;

    println!("The value of the first index is {m}");

    let one = tup.2;

    println!("The value of 3rd index is {one}");

    //Array - allocated on the stack, fixed in size

    let v: [i32; 5] = [1,2,3,4,5,]; //Don't have to declare type and zise, it can be inferred

    let p = [3; 5]; // [3,3,3,3,3]
    
    let first = v[0];

    //Rust's memory safety:

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = v[index]; 
    //Rust calculates that the number input by the user is less than the size of the array, and if it is greater or equal to the size, it panics. This check happens at runtime, so the code will compile. This protects against invalid memory access

    println!("The value of the element at index {index} is: {element}");

}
