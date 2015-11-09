use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;
use std::io::Read;
use std::fmt;

#[derive(RustcDecodable)]
struct Response {
    pub bugs: Vec<BugData>
}

#[derive(RustcDecodable)]
pub struct BugData {
    pub resolution: String,
    pub status: String,
    pub summary: String
}

impl BugData {
    fn new() -> BugData {
        BugData {
            resolution: "unknown".to_string(),
            status: "unknown".to_string(),
            summary: "unknown".to_string()
        }
    }
}

impl fmt::Display for BugData {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{} {}", self.status, self.resolution, self.summary)
    }
}

pub fn get_bug_data(number: &str) -> BugData {
    let api_endpoint = format!("https://bugzilla.mozilla.org/rest/bug/{}?include_fields=summary,status,resolution", number);
    let client = Client::new();
    let mut res = client.get(&api_endpoint)
        .header(Connection::close())
        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let mut response: Response = json::decode(&body).unwrap();
    response.bugs.pop().unwrap_or(BugData::new())
}
