mod reservations {
    use std::collections::HashMap;
    use crate::frontend::date::date;
    use crate::frontend::guests::guests;

    pub struct Table {
        capacity: u32,
        number: u32,
        vip: bool
    }

    struct Reservation<'a> {
        guest: &'a guests::Guest,
        table: &'a Table,
        date: date::Date,
        is_active: bool,
        comment: String
    }
    impl Reservation<'_> {
        fn new<'a>(guest: &'a guests::Guest, table: &'a Table, date: date::Date) -> Reservation<'a> {
            let reservation = Reservation {
                guest,
                table,
                date,
                is_active: true,
                comment: String::from("Unknown")
            };
            reservation
        }

        pub fn cancel(&mut self, reason: &str) {
            self.is_active = false;
            self.comment = reason.to_string();
        }
    }

    struct ReservationOption<'a> {
        date: date::Date,
        table: &'a Table
    }
    impl ReservationOption<'_> {
        fn new(day: u32, hour: u32, minute: u32, table: &Table) -> ReservationOption { 
            ReservationOption {
                date: date::Date::new(day, 01, 2022, hour, minute).unwrap(),
                table: &table
            }
        }
    }

    pub struct ReservationManager<'a> {
        reservations: HashMap<(u32, u32), &'a Reservation<'a> >,
        tables: HashMap<u32, Table>,
        num_tables: u32,
    }
    impl ReservationManager<'_> {
        pub fn reserve_table(&mut self,  guest: &guests::Guest, table: &Table, date: date::Date) {
            let reservation = Reservation::new(guest, table, date);
            self.reservations.insert((date.hour, date.minute), &reservation); 
        }

        pub fn get_free_tables(&self, name: &str, hour: &str, day: &str) -> Vec<ReservationOption<'static>>{
            let free_times: Vec<(u32, u32)> = vec![];
                // Get free entries for the given day/hour
            for minute in vec![0, 30].iter() {
                let hour = hour.parse::<u32>().unwrap();
                match self.reservations.get(&(hour, *minute as u32)) {
                    None => free_times.push((hour, *minute as u32)),
                    _ => ()
                }
            }
            let free_slots: Vec<ReservationOption> = free_times.iter().map(|slot|{
                let table = Table{capacity: 4, number: 12, vip: false};
                ReservationOption::new(20, slot[0], slot[1], &table)
            }).collect();

            free_slots
    }

        fn add_tables(&self, n: u32, capacity: u32, vip: bool) {
            // Get current number of tables
            for i in 1..n {
                let table_num = self.num_tables + i;
                let table = Table{
                    capacity,
                    number: self.num_tables + i,
                    vip
                };
                self.tables.insert((table_num, table))
            }
        }

    }

}


