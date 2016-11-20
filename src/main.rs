#[macro_use] extern crate emojicons;
extern crate hyper;

use hyper::client::Client;

pub mod checklist;


static APPROVED: &'static str = ":white_check_mark:";
static FAILED: &'static str = ":no_entry:";

pub mod notifications {
    use emojicons::EmojiFormatter;

    pub fn approved(text: &str){
        println!("{}  {}", EmojiFormatter(::APPROVED), text);
    }

    pub fn failed(text: &str){
        println!("{}  {}", EmojiFormatter(::FAILED), text);
    }

}

fn main() {
    let url = "http://madewithbytes.com/";
    let client = Client::new();
    let response = client.get(url).send().unwrap();
    checklist::test_is_https_enabled::execute(response);
}
