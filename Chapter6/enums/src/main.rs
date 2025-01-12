fn main() {
    //"IpAddrKind::V4() is a function that takes in a String type, and returns an instance of
    //IpAddr type."
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::0"));

    //Rust does not have a null type. Instead, it uses the Option<T> enum, where T is any type.
    let some_char = Some('e');
    let some_number = Some(5);

    let absent_number: Option<i32> = None;
    
    //Essentially, if there is a situation where a value could be something or nothing, we can
    //encode that as an option type and handle the situation where value is not null, by converting
    //it back to its type, and handling a situation where the value is null, without the type.
}

enum Option<T> {
    None,
    Some(T),
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

//An advantage of Enums, is that all of the potential "structs" which would have different types if
//organised that way, can be passed into a function that accepts the "Message" type.
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColour(i32, i32, i32),
}
//Enums can have methods
impl Message {
    fn call(&self) {
        //body
    }
}

