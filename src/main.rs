#[macro_use] extern crate emojicons;
extern crate hyper;

use hyper::client::Client;

pub mod checklist;


static APPROVED: &'static str = ":white_check_mark:";
static FAILED: &'static str = ":no_entry:";


fn main() {
    let url = "http://madewithbytes.com/";
    let client = Client::new();
    let response = client.get(url).send().unwrap();
    checklist::test_is_https_enabled::execute(response);
}
