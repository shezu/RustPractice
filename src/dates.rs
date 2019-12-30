use std::io::stdin;
use chrono::prelude::*;

pub fn get_distance_in_dates(){
    let mut d1_string = "".to_string();
    println!("Enter a date in (dd/mm/yyyy) format: ");
    stdin().read_line(&mut d1_string).expect("Invalid input");

    let mut d2_string = "".to_string();
    println!("Enter a date in (dd/mm/yyyy) format: ");
    stdin().read_line(&mut d2_string).expect("Invalid input");

    let d1 = NaiveDate::parse_from_str(&d1_string.trim(), "%d/%m/%Y");
    let d2 = NaiveDate::parse_from_str(&d2_string.trim(), "%d/%m/%Y");

    let since = NaiveDate::signed_duration_since;
    let days_difference = since(d1.unwrap(),d2.unwrap()).num_days();
    println!("There are {} days between {} & {}",days_difference.abs(), d1_string.trim(), d2_string.trim());
}