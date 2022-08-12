use std::fmt;



enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}
impl Message {
    fn call(&self) {
        println!("Called fn call on Message")
    }
}


enum IpAddr {
    V4(String),
    V6(String),
}


fn add(a: i32, b: i32) -> i32 {
    return a + b
}

trait Animal {
    fn new(name: &'static str, age: Option<u8>, owner: Option<&Person>, location: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}


// Work with classes
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    nationality: String
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Person {
    fn purchase_cow<'a, 'b>(&'a self, cow: &'b mut Cow<'a>) {
        cow.owner = Some(self)
    }
    fn set_free<'a, 'b>(&'a self, cow: &'b mut Cow<'a>) {
        cow.owner = None
    }
    // fn sell<T: Animal>(animal: & mut Animal, purchaser: &Person) {
    //     animal.owner = purchaser
    // }
}


#[derive(Debug)]
struct Cow<'a> {
    name: &'a str,
    age: u8,
    owner: Option<&'a Person>,
    location: &'a str,
    milk: u8
}
impl fmt::Display for Cow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl<'a> Animal for Cow<'a> {
    fn new(name: &'static str, age: Option<u8>, owner:Option<&Person>, location: &'static str) -> Cow<'a> {
        Cow::<'a> {
            name: name,
            age: if age.is_some() {age.unwrap()} else { 0 },
            owner: owner,
            location: location,
            milk: 100
        }
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn noise(&self) -> &'static str {
        "Mooouuuuhhhh!"
    }
}
impl Cow<'_> {
    pub fn r#move(&mut self, to: &str) {
        self.location = to
    }

    pub fn milk(&self, amount: u8) {
        if amount > self.milk {
            println!("Out of milk. Feed and come back tomorrow")
        } else {
            println!("You got your {} liters of milk. Enjoy!", amount)
        }
    }

}

#[derive(Debug)]
struct Horse<'a> {
    name: &'a str,
    age: u8,
    owner: Option<&'a Person>
}



fn main() {
    let a = 1;
    let b = 2;

    let c = add(a, b);
    println!("{}", c);

    // Work with enums/structs
    let home = IpAddr::V4(String::from("127.0.0.1"));
    // println!("{}", home)
    // format!("Calculated {}", c)

    let m = Message::Write(String::from("Testing"));
    m.call();

    let mut milka: Cow = Cow::new(String::from("Milka"));
    println!("{}", milka);

    // Purchase the cow
    let frank = Person {
       name: "Frank".to_string(),
       age: 15,
       nationality: "German".to_string() 
    };
    frank.purchase_cow(& mut milka);
    println!("The cow milka is currently owned by: {}", milka.owner.unwrap_or(None));
}