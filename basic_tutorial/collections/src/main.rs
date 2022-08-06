use std::collections::HashMap;

fn main() {
    let a = [1,2,3];
    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1,2,3];


    match v2.get(2) {
        Some(n) => println!("The third element is {}", n),
        None => println!("There is no third element!")
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    // Hashmapcarg
    let blue = "Blue".to_string();
    let yellow = "Yellow".to_string();
    
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let score = scores.get(&"Blue".to_string());
    
    let text = "Testing what the test can do to improve the testing of test".to_string();
    word_count(&text)
}

fn word_count(text: &String) {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map)
}