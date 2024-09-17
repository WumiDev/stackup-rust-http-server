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

#[derive(debug)]
enum Version {
    V1_1,
    V2_0,
}

#[derive(debug)]
enum Method {
    Get,
    Post,
    Uninitialised,
}

#[derive(debug)]
struct Route {
    path = String
}