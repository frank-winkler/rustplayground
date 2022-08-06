use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}


fn main() {
    let x = 4;
    let y = MyBox::new(x);

    assert_eq!(4, x);
    assert_eq!(4, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str (Automagicall type coercion through chained deref calls)
    hello(&(*m)[..]); // Dereferences the string and passes the reference to the str slice of the full length of the string 

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}