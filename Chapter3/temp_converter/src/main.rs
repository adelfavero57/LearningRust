use std::io;

fn main() {
    let mut celcius = String::new();


    println!("Please enter a temperature in celcius to be converted");

    io::stdin()
        .read_line(&mut celcius)
        .expect("failed to read line");

    let celcius: f32 = celcius
        .trim()
        .parse()
        .expect("failed to convert to int");

    let far: f32 = (celcius * 9.0/5.0) + 32.0;

    println! ("{celcius} degrees celcius is {far} degrees fahrenheit");
}
