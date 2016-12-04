use hyper::client::response::Response;

use headers;


pub fn execute(response: &Response) -> bool {
    let frame_options = response.headers.get::<headers::XXSSProtection>();
    if frame_options.is_some() {
        ::notifications::approved(&format!("Site provides `X-XSS-Protection: {}`", frame_options.unwrap()))
    } else {
        ::notifications::failed("Site does not provide `X-XSS-Protection` header. (Browser won't stop loading content when they detect XSS).");
    }
    frame_options.is_some()
}
