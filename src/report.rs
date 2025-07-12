use std::collections::HashMap;
use std::fs::write;
use serde::Serialize;

#[derive(Serialize)]
struct Report {
    duplicates: HashMap<String, Vec<String>>,
}

pub fn generate_report(dupes: &HashMap<String, Vec<String>>) {
    let report = Report {
        duplicates: dupes.clone(),
    };
    let json = serde_json::to_string_pretty(&report).unwrap();
    write("report.json", json).unwrap();
}
