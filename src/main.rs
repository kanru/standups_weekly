extern crate hyper;
extern crate rustc_serialize;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

mod api2;
mod bzapi;

fn titlecase(input: &str) -> String {
    input.chars().enumerate()
        .map(|(i, c)| if i == 0 { c.to_uppercase().next().unwrap() } else { c })
        .collect()
}

fn textify(maybe_html: &str) -> String {
    let bug_re =
        Regex::new("<a href=\"http://bugzilla[^\"]+\">[Bb]ug\\s+(?P<number>\\d+)</a>").unwrap();
    let text = bug_re.replace_all(maybe_html, "$number");

    let bug_re =
        Regex::new("(?P<number>\\d{5,})").unwrap();
    bug_re.replace_all(&text, "bug $number")
}

fn extract_bug_numbers(input: &str) -> Vec<String> {
    let bug_re =
        Regex::new("[Bb]ug\\s+(?P<number>\\d+)").unwrap();
    bug_re.captures_iter(input).map(|caps| {
        caps.name("number").unwrap().to_string()
    }).collect()
}

fn extract_bug_details(bugs: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for bug_number in bugs {
        let data = bzapi::get_bug_data(&bug_number);
        result.push(format!("https://bugzil.la/{} {}", bug_number, data));
    }
    result
}

fn main() {
    let decoded = api2::get_project_timeline("perf-tw");

    let mut reports = HashMap::new();

    for status in &decoded {
        let vec = reports.entry(&status.user.name).or_insert(Vec::new());
        vec.push(titlecase(&textify(&status.content)));
    }

    for (username, status) in reports.iter_mut() {
        status.sort();
        status.dedup();

        println!("\n## {} ##\n", username);
        let mut bugs = Vec::new();
        for content in status {
            println!("  * {}", content);
            bugs.extend(extract_bug_numbers(&content));
        }
        if !bugs.is_empty() {
            println!("");
            bugs.sort();
            bugs.dedup();
            let bugs_detail = extract_bug_details(&bugs);
            for bug in bugs_detail {
                println!("  * {}", bug);
            }
        }
    }
}
