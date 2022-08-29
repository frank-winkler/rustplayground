#[derive(Debug)]
struct Animal<'a> {
    name: String,
    age: i32,
    last_fed: std::cell::RefCell<std::time::SystemTime>,
    owner : std::cell::RefCell<Option<&'a Person<'a>>>
    // pub owner: std::cell::RefCell<Option<&'a Person<'a>>>
}
impl <'a> Animal<'a> {
    fn new(name: &str, age: i32) -> Self {
        Animal {
            name: name.to_string(), 
            age,
            last_fed: std::cell::RefCell::new(std::time::SystemTime::now()),
            owner : std::cell::RefCell::new(None)
        }
    }
}

#[derive(Debug)]
struct Person <'a> {
    name: String, 
    age: i32,
    animals: std::cell::RefCell<Vec<&'a Animal<'a>>>
}
impl <'a> Person<'a> {
    fn new(name: &str, age: i32) -> Self {
       Person { 
        name: name.to_string(),
        age,
        animals : std::cell::RefCell::new(vec!())
       } 
    }
    pub fn adopt(&'a self, a: &'a Animal<'a>) {
        let mut owner_borrow = a.owner.borrow_mut();
        *owner_borrow = Some(self);
       let _animal_list = self.animals.borrow_mut();
        
        //animal_list.push(a);
    }
    pub fn feed(&self, a: &Animal) {
        a.last_fed.replace(std::time::SystemTime::now());
    }
    pub fn present_animals(&self) {
        for animal in self.animals.borrow().iter() {
            println!("{:?}", animal)
        }
    }
}

fn main() {
    let dog = Animal::new("Doggy", 12);
    let cat = Animal::new("Kitty", 24);

    let frank = Person::new("Frank", 39);
    frank.adopt(&dog);
    frank.adopt(&cat);
    frank.feed(&dog);
    
}


