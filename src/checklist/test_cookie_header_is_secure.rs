use hyper::header;
use hyper::header::{CookiePair};
use hyper::client::response::{Response};


fn analize_cookie(cookie: &CookiePair) {
    ::notifications::header(&format!("For cookie: `{}`", cookie.name));
    if cookie.secure {
        ::notifications::approved("Cookie is provided securely");
    } else {
        ::notifications::failed("Cookie is provided insecurely. (Should be available only on HTTPS):");
    }
    if cookie.httponly {
        ::notifications::approved("Site sends cookies on HTTP(S) requests only.");
    } else {
        ::notifications::failed("Site sends cookies on any request (Including Javascript).");
    }
}


pub fn execute(response: &Response) -> bool {
    let set_cookie_header = response.headers.get::<header::SetCookie>();
    if set_cookie_header.is_some() {
        let set_cookie_header_contents = set_cookie_header.unwrap();
        for cookie in set_cookie_header_contents.iter() {
            analize_cookie(cookie)
        }
    } else {
        ::notifications::approved("Site does not provide cookies.");
    }
    set_cookie_header.is_some()
}
