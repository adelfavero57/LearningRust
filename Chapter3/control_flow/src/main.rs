fn main() {

    let number = 5;

    if number < 5 {
        println!("Condition true");
    } else {
        println!("Condition false");
    }

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisble by 2");
    } else {
        println!("Not divisible by 4, 3 or 2");
    }

    let condition = true;
    // The value of "number" depends on the branch of the if statement. Both branches of the
    // expression have to evaluate to the same type here.
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");
    

    //Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    
    //Loop lables
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //The following is equivalent to the next commented code
    let mut condition_2 = true;

    loop {
        if condition_2 == true {
            println!("true");
            condition_2 = false;
        } else {
            break;
        }
    }

    // Here:
    condition_2 = true;
    while condition_2 == true {
        println!("true");
        condition_2 = false;
    }

    //using "while" to iterate over an array is error prone.
    
    let a = ['a', 'b', 'c'];
    let mut index = 0;

    while index < a.len() {
        println!("element of array a at index {index} is {}",a[index]);
        index += 1;
    }

    //using "for" is more concise, and decreases the chance of bugs/errors

    for element in a {
        println!("next element is {}",element);
    }

    //"for" is the most commonly used iterator, even in situations where we want to perform a
    //command an arbitrary amount of times.
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");

}
