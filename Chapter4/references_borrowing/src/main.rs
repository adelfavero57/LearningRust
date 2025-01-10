fn main() {

    let mut s = String::from("ello");
    let len = calculate_length_via_borrowing(&s);
    println!("string {s} has length {len}"); //We are able to use the variable even after it has been used in a function, since "main()" still has ownership
    change(&mut s);

    println!("string {s} has length {len}");

    //You can't make two mutable references within the same scope
    //The following won't work:
    //{
        //let r1 = &mut s;
        //let r2 = &mut s;
    //}
    //You also can't make a mutable reference from an immutable variable
    //}
        //let s2 = String::from("Hello");
        //let r1 = &s2
        //let r2 = &mut s2
    //}
    

    //We can't have a mutable reference while we have an immutable one.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}"); //If r1 and r2 are not used after here, there is no problem creating a mutable reference.

    let r3 = &mut s; // no problem
    println!("{r3}");
    

    //println!("{r1} and {r2}"); -> r1 and r2 cannot be used here
}

fn calculate_length_via_borrowing(s: &String) -> usize { //We pass a reference to the function
    s.len() //The function borrows the data and does not take ownership
    //s.push_str(", world");? - This won't work, as we can't modify something we have a reference
    //to.
}

fn change(s: &mut String) { //Passing a mutable reference allows us to modify a string in a function.
    s.push_str(", world");
}

//Rust will not let this code compile. Memory is allocated on the heap within the scope of the
//function, and a reference to it is returned. This means the memory will be freed, but the
//reference returned to the outer scope, leaving a dangling pointer.
//fn dangle() &String -> {
    //let s = String::new()
    //&s
//}
