//We can use if let to replace a match statement, where all we care about is one arm of the match.
//This trades exhaustiveness for conciseness
fn main() {
    println!("Hello, world!");
    let config_max = Some(3u8);

    //OR
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

}

fn config_max(max: Option<u8>) {
    match max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}



