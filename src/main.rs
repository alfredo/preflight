#[macro_use] extern crate emojicons;
extern crate hyper;


use hyper::client::Client;
pub mod checklist;


fn main() {
    let url = "http://madewithbytes.com/";
    let client = Client::new();
    let response = client.get(url).send().unwrap();
    checklist::test_is_https_enabled::execute(response);
}
