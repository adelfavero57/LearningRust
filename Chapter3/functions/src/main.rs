fn main() {
 
    println!("Hello, world!");


    another_function(5, 'h');

    //Statements are instructions that perform an action, and do not return a value
    let _a = 6;

    //Expressions evalueate to a resultant value
    //Examples include: Numeric operations, calling functions, macros
    //Expressions do not need to end in a semicolon
    let _y = {
        let _x = 3;
        _x + 1
    };
    
    let b = five();

    println!("The value of b is {b}");
    
    let c = add_one(b);

    println!("b + 1 is {c}");
}

//You must declare the type of the argument passed to the function
fn another_function(value: i32, unit_label: char) {
    
    println!("The measurement is: {value}{unit_label}");

}

//Functions return the final expression in the function body
fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}
