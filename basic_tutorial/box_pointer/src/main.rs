enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {
    // Stores a reference in stack to an object on the heap
    // Use for:
    // * Object with unknown size at compile time
    // * Transfering ownership of large data blobs without copying
    // * On owned data that you want to have a specific trait, not a specific type (Trait Object)
    let b = Box::new(6);
    println!("b = {}", b);

    // Working with a recursive structure we have to allocate it to the heap
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));



}   
