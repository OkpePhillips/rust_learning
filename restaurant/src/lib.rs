mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::serving::serve_order();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
}

// all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
// If you want to make an item like a function or struct private, you put it in a module.

// pub keyword to make an item public.



mod back_of_house {
    // Adding `pub` to a struct does not automatically make its fields public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {  //public method to create an instance of Breakfast
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    // when an enum is public, all its fields are automatically public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// The `use` keyword can be used to create a shortcut to a path,
// so that we can use the functions therein without specifying the full path everytime.
use crate::front_of_house::hosting; // will let us use all the public functions in the hosting module

// when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path
use std::collections::HashMap;

// after the path, we can specify as and a new local name, or alias, for the type.
use std::io::Result as IoResult;


// To bring different modules from the same crate, we can do this
use std::{cmp::Ordering, io}; // This reduces the number of separate user statements
use std::io::{self, Write}; // This brings std::io and std::io::Write into scope

// The glob operator brings all public items in a path into scope
use std::collections::*; //brings all public items in collections module into scope