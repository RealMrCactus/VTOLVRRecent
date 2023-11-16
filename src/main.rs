extern crate regex;
use regex::Regex;
use std::fs;
use std::collections::HashSet;


fn main() {
    let app_data_path = std::env::var("APPDATA").expect("APPDATA not found");
    let relative_path = "\\..\\LocalLow\\Boundless Dynamics, LLC\\VTOLVR\\Player.log";
    let file_path = format!("{}{}", app_data_path, relative_path);

    // Read the file
    let text = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    // Regular expression to strictly match player entries
    let re = Regex::new(r"(\d+),([\w\d]+),([\w\d]+),(?:[\w\d]+,){3}[\w\d]+;").unwrap();

    // HashSet to store and check for duplicates (using a tuple of id, name, team)
    let mut seen = HashSet::new();

    // Find all matches and process them
    let mut found_any = false;
    for cap in re.captures_iter(&text) {
        let id = cap[1].to_string();
        let name = cap[2].to_string();
        let team = cap[3].to_string();

        if seen.insert((id.clone(), name.clone(), team.clone())) {
            found_any = true;
            println!("ID: {}, Name: {}, Team: {}", id, name, team);
        }
    }

    if !found_any {
        println!("No player data found in the log.");
    }
}