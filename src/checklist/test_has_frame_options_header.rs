use hyper::client::response::Response;

use headers;


pub fn execute(response: &Response) -> bool {
    let frame_options = response.headers.get::<headers::XFrameOptions>();
    if frame_options.is_some() {
        ::notifications::approved(&format!("Site provides click-jack configuration `X-Frame-Options: {}`", frame_options.unwrap()))
    } else {
        ::notifications::failed("Site does not provide click-jack configuration `X-Frame-Options`. (It can be embedded in any site)");
    }
    frame_options.is_some()
}
