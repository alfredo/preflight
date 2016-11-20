use hyper::header;
use hyper::client::response::Response;


pub fn execute(response: &Response) -> bool {
    let content_type = response.headers.get::<header::ContentType>();
    if content_type.is_some() {
        ::notifications::approved(&format!("Site defines `Content-Type: {}`", content_type.unwrap()));
    } else {
        ::notifications::failed("Site does not define `Content-Type`");
    }
    content_type.is_some()
}
