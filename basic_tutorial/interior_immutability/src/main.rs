use std::{rc::Rc, cell::{RefCell, Ref}};


pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }  

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let perc_of_max = self.value as f64 / self.max as f64;
        if perc_of_max >= 1.0 {
            self.messenger.send("Error: All up")
        } else if perc_of_max >=0.9 {
            self.messenger.send("Urgent Warning: Allmost over")
        } else if perc_of_max >= 0.75 {
            self.messenger.send("Warning: Almost over")
        }
    }
}

// We construct a mock-messenger object to track the number of calls on the messenger that was used in the program

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>,>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // Transforming the underlying mutable value is not allowed through a immutable reference
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
    }

    
   #[test]
   fn it_sends_correct_num_messages() {
      let messenger = MockMessenger::new();
      let tracker = RefCell::new(LimitTracker::new(&messenger, 100));

      tracker.borrow_mut().set_value(80);

      assert_eq!(messenger.sent_messages.borrow().len(), 1);
   }
    
}



fn main() {
    refcell_with_list();
    conflicts_immutability_rules();
}

fn refcell_with_list() {
    // >>>>>>> APPLYING RefCell pointers for multiple ownership
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


}

fn conflicts_immutability_rules() {
    // At compile time we can not have a immutable reference to a mutable variable
    // This could be solved with some indirection within the data structure calling methods to mutate the data in the structure from outside

    // Uncomment to check compile time errors >>>>
    // let a = 5;
    // let b = &mut a;

    // let mut c = 10;
    // let d = &c;
    // *d = 10;
}
