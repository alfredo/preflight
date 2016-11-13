extern crate hyper;

use hyper::client::Client;

fn main() {
    let url = "http://madewithbytes.com/";
    let client = Client::new();
    let res = client.get(url).send().unwrap();
    println!("URL: {}", res.status);
}
