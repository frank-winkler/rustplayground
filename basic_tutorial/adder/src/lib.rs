#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name) 
}

struct Guess {
    value: i32,
}
impl Guess {

    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Your guess must be greater than or equal to 1. Value was `{}`.", value)
        } if value > 100 {
            panic!("Your guess must be smaller or equal to 100. Value was `{}`.", value) 
        }
        Guess{ value }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    #[should_panic(expected="Your guess must be greater than or equal to 1")]
    fn guess_below_one() {
        let guess = Guess::new(0);
    }

    #[test]
    #[should_panic(expected="Your guess must be smaller or equal to 100. Value was `101`.")]
    fn guess_below_100() {
        let guess = Guess::new(101);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle{height: 30, width: 40};
        let rect2 = Rectangle{height: 20, width: 30};

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let rect1 = Rectangle{height: 20, width: 30};
        let rect2 = Rectangle{height: 30, width: 40};
        
        assert!(!rect1.can_hold(&rect2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was : `{}`",
            result
        )
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus tow does not equal four"))
        }
    }

}
