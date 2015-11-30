use chrono::Local;
use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;
use std::io::Read;
use std::fmt::Display;

#[derive(RustcDecodable, Debug)]
pub struct User {
    pub username: String,
    pub name: String,
    pub slug: String,
}

#[derive(RustcDecodable, Debug)]
pub struct Project {
    pub name: String,
    pub slug: String,
}

#[derive(RustcDecodable, Debug)]
pub struct Status {
    pub user: User,
    pub project: Project,
    pub content: String,
}

pub fn get_project_timeline(slug: &str, day: &str) -> Vec<Status> {
    let day: Box<Display> = if day.is_empty() {
        Box::new(Local::today().format("%Y-%m-%d"))
    } else {
        Box::new(day)
    };
    let api_endpoint = format!("http://www.standu.ps/api/v2/statuses/project_timeline.\
                                json?slug={}&week={}&count=800",
                               slug, day);
    let client = Client::new();
    let mut res = client.get(&api_endpoint)
                        .header(Connection::close())
                        .send()
                        .unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    json::decode(&body).unwrap()
}
