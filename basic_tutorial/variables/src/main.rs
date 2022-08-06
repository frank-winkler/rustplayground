
fn main() {

    basic_syntax();
    base_function();

    let f = add(12, 45);
    println!("12 and 45 make : {}", f);

    count_to(100);

    count_down(100);

    print_series();
}

fn basic_syntax() {
    let x = 5;
    println!("{}", x);
    let x = 10;
    println!("{}", x);

    const CONSTANT: u32 = 100_000;

    // Integers
    let a = 98_222; // Decimal
    let b = 0xff; //Hex
    let c = 0o77; //Octal
    let d = 0b1111_0000; //Binary
    let e = b'A'; //Byte (u8 only)

    // Floating-point
    let f = 2.0;
    let f: f32 = 2.5;

    let sum = 3 + 5;
    let diff = 3.5 - 3.2;
    let product = 3.4 * 12.0;
    let product = 3 * 12;
    let quotient = 56.0/32.0;
    let remainder = 43 % 4;
    
    // Booleans
    let t = true;
    let f = false;

    // Characters
    let c = 'z';
    let z = 'Z';


    // TUPLES
    let tup = ("Let's get rusty", 100);
    let(channel, subcount) = tup;
    let subcount = tup.0;

    let error_codes = [200, 300, 404, 500];
    let not_found = error_codes[2];

    let byte = [0; 8];
}

fn base_function() {
    println!("Writting from base_function!");
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn count_to(n: i32) {
    let mut counter = 0;
    loop {
        println!("Counting: {}", counter);
        counter += 1;
        if counter == n {
            println!("You reached count of {}", n);
            break;
        }
    }
}

fn count_down(n: i32) {
    let mut counter = n;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }

    println!("LIFTOFF!!!!");
}


fn print_series() {
    let ab = [1,2,3];
    for item in ab.iter() {
        println!("Item  : {}", item);
    }

    for n in 1..5 {
        println!("{}", n);
    }
}