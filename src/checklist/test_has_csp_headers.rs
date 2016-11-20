use hyper::client::response::Response;


pub fn execute(response: &Response) -> bool {
    let csp_headers = response.headers.get_raw("Content-Security-Policy");
    let csp_headers_report = response.headers.get_raw("Content-Security-Policy-Report-Only");
    if csp_headers.is_some() {
        ::notifications::approved("Site provides CSP headers");
    } else if csp_headers_report.is_some() {
        ::notifications::warning("Site CSP headers are in report-only mode");
    } else {
        ::notifications::failed("Site does not provide CSP headers");
    }
    csp_headers.is_some()
}
