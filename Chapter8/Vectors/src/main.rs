fn main() {
    let v: Vec<i32> = Vec::new();

    // We don't have to initialize the type here, since we are inserting integers.
    // We use the vec! macro to create the vector
    let v1 = vec![1,2,3];

    // We don't need typ annotation here, because the elements we've added are all i32.
    let mut v2 = Vec::new();
    v2.push(4);
    v2.push(5);
    v2.push(6);

    // There are two ways to reference an item in a vector, the first panics when the index is out of range, the second handles the scenario.
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let mut v3 = vec![1,2,3,4];

    let mut first = &v3[0];

    v3.push(6);

    // println!("The first element is: {first}") - We can't uncomment this line, because we can't hold a refer to a mutable reference, while we hold an immutable reference to that item.
    println!("{first}")

    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    // we cannot add or remove from the vector, as the for loop holds a reference to v, preventing modifications
    for i in &mut v {
        *i += 50
        println!("{i}");
    }
    
    //We can use an enum type as a way to store multiple types within a Vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}