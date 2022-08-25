mod backofhouse;
mod frontofhouse;
mod utils;
use backofhouse::cooking;
use backofhouse::cooking::{Slicing, Frying};
use frontofhouse::reservation;


fn main() {

    // Set up the restaurant
    let working_days = vec!["Monday", "Tuesday", "Wednesday"];
    let mut calendar = reservation::Calendar::new(5, 30, working_days);
    // Get reservations
    calendar.reserve("Monday", 1800, 4);
    calendar.reserve("Monday", 2000, 4);
    calendar.reserve("Monday", 1200, 4);
    calendar.reserve("Monday", 1200, 4);
    calendar.list_reservations("Monday");
    // Get free reservation slots


    // Get a potatoe
    let mut potatoe: cooking::Ingredient = cooking::Ingredient::new("Potatoe".to_string());
    // Slice it
    potatoe.slice(34);
    // Fry it
    potatoe.fry(25);

    // Call it in ready
    println!("{:?}", potatoe);
    println!("{}",vec![1.0,2.0,3.0].contains(&1.0))
}
