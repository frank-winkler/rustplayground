pub mod guests {

    use std::collections::HashMap;
    use crate::frontend::guests::guests;

    pub struct Guest {
        name: String,
        birthdate: String,
        age: u32,
    }

    impl Guest {
        fn new(name: &str, age: u32, birthdate: &str ) -> Guest {
            Guest {
                name: name.to_string(),
                age,
                birthdate: "11.12.2022".to_string()
            }
        }
    }

    pub struct Guestregistry {
        guests: HashMap<String, Guest>,
    }
    impl Guestregistry {
        
        fn new() -> Guestregistry {
            Guestregistry{
                guests: HashMap::new()
            }
        }
        fn register(&self, name: &str) {
            let guest = guests::Guest::new(name, 0, "12.03.1982");
            self.guests.insert(name.to_string(), guest);

        }

        fn get(&self, name: &str) -> &Guest {
            let guest = match self.guests.get(name) {
                Some(guest) => guest,
                None => &Guest::new(name, 0, "01.18.1946")
            };
            return guest
        }
    }
}