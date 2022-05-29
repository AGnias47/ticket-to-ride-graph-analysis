use std::fs;

pub fn valid_cities() -> Vec<String> {
    let data = fs::read_to_string("mattgawarecki-ticket-to-ride/usa.cities.json")
        .expect("Unable to read file");
    return serde_json::from_str::<Vec<String>>(&data).unwrap();
}

pub fn valid_colors() -> Vec<String> {
    let data = fs::read_to_string("mattgawarecki-ticket-to-ride/colors.json")
        .expect("Unable to read file");
    return serde_json::from_str(&data).unwrap();
}
