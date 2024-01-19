mod back_of_house;
mod front_of_house;
pub use crate::back_of_house::{Appetizer, Breakfast};
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    hosting::add_to_waitlist();
}

fn deliver_order() {}
