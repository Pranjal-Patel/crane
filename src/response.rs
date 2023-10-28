/// Response builder.
///
/// # Examples
///
/// ```rs
/// use crane_webserver::webserver::WebServer;
/// fn main() {
///     // .. server setup
/// }
///
/// fn root(_: Query) -> Response {
///     ResponseBuilder::new()
///         .status(200)
///         .header("Content-Type", "text/plain")
///         .body("Hello, World!")
///         .build()
/// }
/// ```
#[derive(Debug)]
pub struct ResponseBuilder {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl ResponseBuilder {
    /// Construct a new `ResponseBuilder`.
    pub fn new() -> Self {
        ResponseBuilder {
            status: 200,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    /// Set the html status code.
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    /// Set a header key-value pair.
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    /// Set the body of the response.
    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    /// Consume the `ResponseBuilder` and construct a `Response`.
    pub fn build(self) -> Response {
        Response {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// The response which will be sent when requested.
pub struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<_>>()
            .join("\r\n");

        write!(
            f,
            "HTTP/1.1 {} {}\r\n{headers}\r\n\r\n{}",
            self.status, "OK", self.body
        )
    }
}