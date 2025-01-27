fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five {} six {}", five.unwrap(), six.unwrap());

    let q = Coin::Quarter(UsState::Alabama);
    
    value_in_cents(q);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Nebraska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {state:?}");
                25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

