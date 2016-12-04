extern crate emojicons;
#[macro_use] extern crate hyper;

pub mod checklist;
pub mod headers;

use hyper::client::Client;
use hyper::client::response::Response;

static APPROVED: &'static str = ":white_check_mark:";
static FAILED: &'static str = ":no_entry:";
static WARNING: &'static str = ":warning:";

pub mod notifications {
    use emojicons::EmojiFormatter;

    pub fn approved(text: &str) {
        println!("{}  {}", EmojiFormatter(::APPROVED), text);
    }

    pub fn failed(text: &str) {
        println!("{}  {}", EmojiFormatter(::FAILED), text);
    }

    pub fn warning(text: &str) {
        println!("{}  {}", EmojiFormatter(::WARNING), text);
    }

}

fn main() {
    let url = "http://madewithbytes.com/";
    let client = Client::new();
    let response: Response = client.get(url).send().unwrap();
    checklist::test_url_is_https::execute(&response);
    checklist::test_has_csp_headers::execute(&response);
    checklist::test_has_content_type_header::execute(&response);
    checklist::test_has_frame_options_header::execute(&response);
    checklist::test_has_xss_protection_header::execute(&response);
}
