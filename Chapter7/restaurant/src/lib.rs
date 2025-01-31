// front_of_house does not need to be public, because eat_at_restaurant() is a sibling
mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //Using super to call the parent module (in this case crate)
        super::deliver_order();
    }

    fn cook_order() {}
    
    //making a struct public does not make the fields public.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    //making an enum public makes all of its options public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
    
    //Shortening the full path, using the 'use' keyword
    hosting::add_to_waitlist();
    
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //meal.seasonal_fruit = String::from("blueberries"); This line won't compile, as we can't see
    //the seasonal_fruit 

}
