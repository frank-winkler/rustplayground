use own_project::frontend;

#[cfg(test)]
mod tests {
    use own_project::frontend;


    #[test]
    fn reservation_created_for_new_guest() {
        let manager = frontend::ReservationManager{};
        
        let guest = frontend::Guest::new("Frank Winkler");
        
        manager.reserve_table()
        
        assert!(true)
    }
}
