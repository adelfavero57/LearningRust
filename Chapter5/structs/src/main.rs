fn main() {
    let user1 = User { //set variable to mutable, to modify fields after creation
        active: true,
        email: String::from("abc123@example.com"),
        username: String::from("abc123"),
        sign_in_count: 0,
    };

    println!("user1's email is {}", user1.email);
    
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    //println!("user1's username is {}", user1.username); - This line triggers a borrw after move
    //error, since the data from user1 was moved to user2. if we gave user2 a new username, then we
    //could still use user1, since sign_in_count and active have the copy trait.

    //Tuple structs have a name, but no field names, only field types
    let black = Colour(0, 0, 0)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// The built in "field init shorthand" syntax allows us to not have to retype the arguments passeed
// to the function.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Colour(i32, i32, i32);
