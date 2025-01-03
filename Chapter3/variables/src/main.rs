fn main() {
    //mutability
    let mut x = 5;
    
    println!("The value of x is: {x}");
    
    x = 6;

    println!("The value of x is: {x}");
    
    //constants: must be typed, must not be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing allows to use the same variable name, for two different values while keeping the
    //variable immutable. It also allows to change the type of a variable.
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is: {y}");
}
