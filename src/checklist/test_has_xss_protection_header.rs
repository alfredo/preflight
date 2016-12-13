use hyper::client::response::Response;

use headers;


pub fn execute(response: &Response) -> bool {
    let xss_protection = response.headers.get::<headers::XXSSProtection>();
    if xss_protection.is_some() {
        ::notifications::approved(&format!("Site provides `X-XSS-Protection: {}`", xss_protection.unwrap()))
    } else {
        ::notifications::failed("Site does not provide `X-XSS-Protection` header. (Browser won't stop loading content when they detect XSS).");
    }
    xss_protection.is_some()
}
