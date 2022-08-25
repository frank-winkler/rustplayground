use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Calendar {
    seats: i8,
    max_duration: i8,
    availabilities: HashMap<i16, i8>,
    reservations: HashMap<&'static str,Vec<(i16, i8)>>,
}
impl Calendar {
    pub fn new(seats: i8, max_duration: i8, working_days: Vec<&'static str>) -> Self {
        // Create a day entry for each working day
        let mut reservation_dict = HashMap::new();
        for i in working_days {
            reservation_dict.insert(i, vec![]);
        }
        
        Calendar {
            seats,
            availabilities: HashMap::new(),
            max_duration,
            reservations : reservation_dict
        }
    }

    fn is_available(&mut self, day: &'static str, time: i16) -> bool {
         match self.reservations.entry(day) {
            Entry::Vacant(..) => { true },
            Entry::Occupied(e) => {
                for entry in e.get().iter() {
                    if entry.0 == time {
                        return false
                    }
                }
                return true
            }
         }
    }

    pub fn reserve(&mut self, day: &'static str, time: i16, people: i8) -> bool {
        if self.is_available(&day, time) {
            match self.reservations.entry(day) {
                Entry::Vacant(e) => { e.insert(vec![]);},
                Entry::Occupied(mut e) => { e.get_mut().push((time, people))}
            }
            *self.seats.entry(time).or_insert(0) += people;
            return true
        }
        false
    }

    pub fn list_reservations(&mut self, day: &'static str) {
        let reservations = &self.reservations[day];
    //    Access and print out existing reservations
        for reservation in reservations {
            println!("{:?}", reservation)
        }
    }

    pub fn get_availabilities(&self, day: &'static str, time: i16) {
        if !self.seats.contains_key(&time) {
            return self.
        }
    }
}