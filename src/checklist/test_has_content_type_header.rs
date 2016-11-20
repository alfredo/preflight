use hyper::client::response::Response;


pub fn execute(response: &Response) -> bool {
    let content_type_header = response.headers.get_raw("Content-Type");
    if content_type_header.is_some() {
        ::notifications::approved("Site defines `Content-Type`");
    } else {
        ::notifications::failed("Site does not define `Content-Type`");
    }
    content_type_header.is_some()
}
