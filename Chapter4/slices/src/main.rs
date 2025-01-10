fn main() {
    
    let s = String::from("hello world");
    let string_literal = "hello world"; //type "str"

    let hello = &s[0..5]; //first in range is the starting index of the slice, second in range is the ending index + 1.
    let world = &s[6..11];

    let word = first_word(&s);
    let word2 = first_word(string_literal);

    //s.clear() - This will cause a compiler error, as .clear() needs to take a mutable reference,
    //and we already have an immutable reference to s, which is used again after this line.
    
    println!("{word}");
    println!("word2 is {word2}");
}
//Returns the first word of a string, indicated by a space.
//Slices are string literals, they're type is "str"
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); //Converts string to byte array

    for (i, &item) in bytes.iter().enumerate() { //iter(), returns each element of a collection, enumerate wraps these in a tuple. The first element of the tuple is the index, the second element is the item.
        if item == b' ' { //Searches for the byte literal space
            return &s[0..i];
        }
    }
    &s[..]
}

