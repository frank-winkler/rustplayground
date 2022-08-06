use add_one;
use add_two;
use rand;


fn main() {
    let x = 1;
    let y = add_one::add_one(x);

    println!("Caling add-one with {} results in {}", x, y);

    let z = add_two::add_two(x);
    println!("Calling add-two with {} results in {}", x, z);
}
