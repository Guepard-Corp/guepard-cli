use reqwest::{get, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client};

/// Construct a template HeaderMap for sending public API requests.
pub fn build_basic_headers(bearer_token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(bearer_token).expect("Invalid Bearer Token"),
    );
    headers
}
