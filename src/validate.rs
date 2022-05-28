pub fn valid_cities() -> Vec<String> {
    return serde_json::from_str::<Vec<String>>("mattgawarecki-ticket-to-ride/usa.cities.json").unwrap();
}

pub fn valid_colors() -> Vec<String> {
    return serde_json::from_str("mattgawarecki-ticket-to-ride/colors.json").unwrap();
}
