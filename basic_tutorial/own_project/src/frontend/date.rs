pub mod date {
    pub struct Date {
        pub hour: u32, 
        pub minute: u32,
        pub day: u32,
        pub month: u32, 
        pub year: u32,
    }
    impl Date {
        pub fn new(day: u32, month: u32, year: u32, hour: u32, minute: u32) -> Result<Date, &'static str> {
            if day > 31 || day < 1 { return Err("Day must be within 1 to 31")}
            if month > 12 || month < 1 { return Err("Month must be within 1 to 12")}
            if year < 2022 || year > 2023 { return Err("Only accept reservations for 2022/2023")}
            if hour < 1 || hour > 24 { return Err("Hour out of bounds")}
            if minute < 1 || minute > 60 { return Err("Minute out of bounds")}

            Ok(Date {
                day,
                month,
                year,
                hour,
                minute
            })
        }
        fn from_string(string: &str) -> Vec<&str>{
            string.split(&['-', ' ', ':', '@'][..]).collect()
        }
    }
}