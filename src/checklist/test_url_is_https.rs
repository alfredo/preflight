use hyper::client::response::Response;


pub fn execute(response: Response) -> bool {
    let is_https = response.url.scheme() == "https";
    if is_https {
        ::notifications::approved("Site uses HTTPS");
    } else {
        ::notifications::failed(" Does not use HTTPS");
    }
    is_https
}
