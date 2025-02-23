fn main() {

    //Append
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{s}");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2} and s1 is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //s1 is no longer valid
    println!("s3 is {s3} and s2 is {s2}");

    //.format is similar to println! macro, but it returns a String 
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //You can't index a String with square brackets, like you would other programming languages. This is because Rust strings support UTF-8, which means some characters take more than one byte to store.
    
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    let world = "world";
    let w = &world[0..1];

    println!("first 4 bytes of hello is {s}, first byte of world is {w}");

    //iterating through a string
    for c in hello.chars() {
        println!("{c}");
    }
}
