use hyper::header;
use hyper::client::response::Response;


pub fn execute(response: &Response) -> bool {
    let content_type_header = response.headers.get::<header::ContentType>();
    if content_type_header.is_some() {
        ::notifications::approved(&format!("Site defines `Content-Type: {}`", content_type_header.unwrap()));
    } else {
        ::notifications::failed("Site does not define `Content-Type`");
    }
    content_type_header.is_some()
}
