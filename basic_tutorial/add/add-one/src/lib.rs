use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = 1;
        let result = add_one(x);

        assert_eq!(result, 2);
    }
}