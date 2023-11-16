extern crate regex;
use regex::Regex;
use std::fs;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

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
    let mut output = String::new();

    for cap in re.captures_iter(&text) {
        let id = cap[1].to_string();
        let name = cap[2].to_string();
        let team = cap[3].to_string();
    
        if seen.insert((id.clone(), name.clone(), team.clone())) {
            found_any = true;
            output.push_str(&format!("ID: {}, Name: {}, Team: {}\n", id, name, team));
        }
    }
    
    if !found_any {
        output.push_str("No player data found in the log.\n");
    }
    fs::write("output.txt", output.clone()).expect("Unable to write to file");
    println!("{}", output);
    std::io::stdin().read_line(&mut String::new()).unwrap();
}