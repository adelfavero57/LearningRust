fn main() {
    let string = "String literal";
    println!("{string}");
    
    //The String type manages data allocation on the heap, and is able to store data that is not
    //known at compile time.
    //Rust will automatically return memory to the heap when the variable goes out of scope.
    //When a variable goes out of scope, rust calls the "drop" function automatically to free the
    //memory.
    {
        let mut s = String::from("hello");
        s.push_str(", world");
        println!("{s}!");
    } //"s" goes out of scope after this line
    
    
    let mut s1 = String::from("hello");
    {
        let s2 = s1;
    }
    //s1 is considered invalid (and subsequently
    //freed/dropped) when its assigned to s2. Since the pointer to the heap is copied and deleted,
    //this is called a "move" instead of a shallow copy

    //s2 is local to the above scope, and does not exist
    //outside of that.
    
    //The inverse is true as well. Assigning new data to an existing variable will free that memory
    //(if it is allocated on the heap)
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let a = 1;

    {
        let b = a;
    }
    println!("{a}");

    // We can use .clone() to deep copy, though this can be very expensive.
    let mut s1 = String::from("hello");
    {
        let s2 = s1.clone();
    }
    println!("{s1}");

    let s3 = String::from("hello again");

    let s4 = takes_and_gives_back(s3);

    println!("{s4}");
    
    let s5 = String::from("Stringy");
    let (s6, len) = calculate_length(s5);
    println!("{s6} has length {len}");
}

//This function takes ownership of s3 and gives the string back
fn takes_and_gives_back(input: String) -> String {
    input
}

//Return multiple values
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
