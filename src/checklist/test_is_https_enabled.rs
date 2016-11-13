use emojicons::EmojiFormatter;
use hyper::client::response::Response;


pub fn execute(response: Response) -> bool {
    let is_https = response.url.scheme() == "https";
    if  is_https {
        println!("Site uses HTTPS {}", EmojiFormatter(":white_check_mark:"))
    } else {
        println!("{}  Does not uses HTTPS", EmojiFormatter(":no_entry:"))
    }
    is_https
}
