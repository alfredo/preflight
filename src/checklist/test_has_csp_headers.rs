use hyper::client::response::Response;

use headers;


pub fn execute(response: &Response) -> bool {
    let csp = response.headers.get::<headers::ContentSecurityPolicy>();
    let csp_report = response.headers.get::<headers::ContentSecurityPolicyReportOnly>();
    if csp.is_some() {
        ::notifications::approved("Site provides CSP headers");
    } else if csp_report.is_some() {
        ::notifications::warning("Site CSP headers are in report-only mode");
    } else {
        ::notifications::failed("Site does not provide CSP headers (Any dynamic resource can be loaded).");
    }
    csp.is_some()
}
