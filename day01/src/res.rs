pub mod res {

    pub fn get_data() -> String {
        let bytes = include_bytes!("res/day1.txt");
        return String::from_utf8_lossy(bytes).to_string();    
    }

    pub fn get_example_data() -> String {
        let bytes = include_bytes!("res/day1-example.txt");
        return String::from_utf8_lossy(bytes).to_string();    
    }
}