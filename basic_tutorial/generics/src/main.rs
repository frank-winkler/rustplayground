
#[derive(Debug)]
enum Point {
    IntegerPoint(i32, i32),
    FloatingPoint(f64, f64),
}

fn main() {

    let char_list = vec!['a', 'c', 'd'];
    let num_list = vec![1,4,5,3,6];

    println!("The largest char is {}", get_largest(char_list));
    println!("The largest number is {}", get_largest(num_list));

    let int_point = Point::IntegerPoint(23, 45);
    let f32_point = Point::FloatingPoint(34.44, 23.44);

    for point in vec![int_point, f32_point] {
        println!("The point {:?} was created successfully using types", point)
    }
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest: T = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn traits_in_function_signatures() {

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.summarize_author(), self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    // Implements traits in function signatures
    pub struct Author {
        name: String,
        age: i32,
        num_articles: i32
    }

    pub struct Article <T> {
        content: T,
        name: String,
    }

    pub trait Summary {
        fn summarize_author(&self) -> String; 
        fn summarize(&self) -> String {
            String::from("Read more ...")
        }
    }

    pub fn notfiy(item1: &impl Summary, item2: &impl Summary) {
            item1.summarize_author();
            item2.summarize();
    }

    pub fn notify<T: Summary>(item1: &T, items2: &T) {

    }




}

