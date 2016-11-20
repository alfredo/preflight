use hyper::client::response::Response;

use headers;

pub fn execute(response: &Response) -> bool {
    let csp_headers = response.headers.get::<headers::ContentSecurityPolicy>();
    let csp_headers_report = response.headers.get::<headers::ContentSecurityPolicyReportOnly>();
    if csp_headers.is_some() {
        ::notifications::approved("Site provides CSP headers");
    } else if csp_headers_report.is_some() {
        ::notifications::warning("Site CSP headers are in report-only mode");
    } else {
        ::notifications::failed("Site does not provide CSP headers");
    }
    csp_headers.is_some()
}
