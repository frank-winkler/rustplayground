mod backofhouse;
mod utils;
use backofhouse::cooking;
use backofhouse::cooking::{Slicing, Frying};

fn main() {
    let a: cooking::Dish = cooking::Dish::new("Testing".to_string());

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
