use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    //insert function will add to the hashmap
    //types are implied here, and are homogenious. For this hashmap, all keys are Strings, and all values are i32. 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // .get() to access the value, which returns an Option<&v>. .get() will return None if there is no value. We handle that using .copied() to get an Option<i32> rather than Option<&i32> then .unwrap_or() to set the score value to 0, if no value is present.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //inserting a variable that does not implement the copy trait, into the hashmap, moves the variable. The original reference cannot be used.
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let n = map.get(&String::from("Favourite colour"));

    println!("{n:?}");

    //Only one value per key can be stored. There are multiple ways to handle adding a new value to a key: Overwriting the old, Ignore the new value, or combine the two

    //The default behaviour or .insert() is to overwite.
    
    //.or_insert() will only add value if there is not a value already there.
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}")
}