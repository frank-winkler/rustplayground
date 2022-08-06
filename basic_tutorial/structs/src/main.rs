struct User {
    username: String,
    email: String,
    signed_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32
} 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self. width > other.width && self.height > other.height
    }
}
fn main() {
    let user1 = User {
        username: String::from("Frank"), 
        email: "frank.winkler@crayon.com".to_string(), 
        signed_in_count: 0,
         active: true
    };
    describe_user(user1);

    let user2 = build_user("testing", "whatever");
    describe_user(user2);

    // Playing with Rectangles
    let rect1 = Rectangle{width: 4, height: 10};
    println!("The area of react1 is: {}", rect1.area());
    println!("{:#?}", rect1);

    let rect2 = Rectangle{width: 2, height: 8};
    let rect3 = Rectangle{width: 5, height: 8};
    let rect4 = Rectangle{width: 3, height: 11};

    for rect in [rect2, rect3, rect4] {
        println!("Rectangle 1 can hold {:?}? {}", rect, rect1.can_hold(&rect));
    }
}

fn describe_user(user: User) {
    println!("{} is registered with email {} has signed in {} times and is currently {}", 
    user.username, user.email, user.signed_in_count, user.active);
}

fn build_user(email: &str, username: &str) -> User {
    User { username: username.to_string(), email: email.to_string(), signed_in_count: 0, active: true }
}
