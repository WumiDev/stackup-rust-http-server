use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(debug)]
pub struct HttpRequest {
    method: Method,
    route: Route,
    version: Version,
    headers: HttpHeader,
    request_body: String
}

#[derive(debug)]
struct HttpHeader {
    headers: Hashmap<String, String>
}

impl HttpHeader {
    pub fn new(request: &str) -> Option<HttpHeader> {
        // let mut httpheader: HttpHeader = HttpHeader {
        let mut httpheader = HttpHeader {
            headers: HashMap::new(),
        };

        // Split request into header part and body part
        // let (_, header_str: &str) = request.split_once(delimiter: "\r\n")?;
        let (_, header_str) = request.split_once("\r\n")?;
        
        // Iterate over each line in the header part
        // for line: &str in header_str.split_terminator("\r\n") {
        for line in header_str.split_terminator("\r\n") {
            if line.is_empty() {
                break;
            }

            // Split the line into header and value
            // let (header: &str, value: &str) = line.split_once(delimiter: ":")?;
            let (header, value) = line.split_once(":")?;
            
            
            // Insert the header and value into the HashMap
            // httpheader HttpHeader
            // .headers HashMap<String, String>
            // .insert(k: header.trim().to_string(), v: value.trim().to_string());
            httpheader.headers.insert(header.trim().to_string(), value.trim().to_string());
        }

        Some(httpheader)
    }
}

#[derive(debug)]
enum Version {
    V1_1,
    V2_0,
}

#[derive(debug)]
struct VersionError {
    msg: String,
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Version {
    pub fn new(request: &str) -> Result<Self, VersionError> {
        Version::from_str(request)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(request: &str) -> Result<Self, Self::Err> {
        let request_split = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let splits = method_line.split_ascii_whitespace();
            for split in splits {
                if split == "HTTP/1.1" {
                    return Ok(Version::V1_1);
                } else if split == "HTTP/2" || split == "HTTP/2.0" {
                    return Ok(Version::V2_0);
                }; 
            }
        };
        let invalid = format!("Unknown protocol version in {}", request);
        let version_error = Version { msg: invalid};
        Err(version_error)
    }
}

#[derive(debug)]
enum Method {
    Get,
    Post,
    Uninitialised,
}

impl Method {
    pub fn new(request: &str) -> Method {
        let request_split = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let method_line = method_line.split_once(' ');
            if let Some((method, _rest)) = method_line {
                return match method {
                    "GET" => Method::Get,
                    "POST" => Method::Post,
                    _ => Method::Uninitialised,
                }
            };
        };
    Method::Uninitialised
}

#[derive(debug)]
struct Route {
    path = String
}