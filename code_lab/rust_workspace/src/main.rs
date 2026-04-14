
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
}

#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_text: String,
    body: String,
}

impl HttpResponse {
    fn to_string(&self) -> String {
        format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code, self.status_text, self.body.len(), self.body)
    }
}

fn parse_request(raw: &str) -> Option<HttpRequest> {
    let first_line = raw.lines().next()?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() != 3 { return None; }
    Some(HttpRequest {
        method: parts[0].into(),
        path: parts[1].into(),
        version: parts[2].into(),
    })
}

fn handle_request(req: &HttpRequest) -> HttpResponse {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => HttpResponse {
            status_code: 200, status_text: "OK".into(),
            body: "<h1>Daithon Server</h1>".into(),
        },
        ("GET", "/health") => HttpResponse {
            status_code: 200, status_text: "OK".into(),
            body: "{\"status\": \"alive\"}".into(),
        },
        ("GET", _) => HttpResponse {
            status_code: 404, status_text: "Not Found".into(),
            body: "<h1>404</h1>".into(),
        },
        _ => HttpResponse {
            status_code: 405, status_text: "Method Not Allowed".into(),
            body: "".into(),
        },
    }
}

fn main() {
    let req1 = parse_request("GET / HTTP/1.1\r\nHost: localhost\r\n");
    let t1 = req1.is_some() && req1.as_ref().unwrap().method == "GET";
    println!("TEST_parse|{}|{:?}", if t1 { "PASS" } else { "FAIL" }, req1);

    let resp1 = handle_request(&req1.unwrap());
    let t2 = resp1.status_code == 200;
    println!("TEST_root|{}|status={}", if t2 { "PASS" } else { "FAIL" }, resp1.status_code);

    let req_404 = parse_request("GET /nope HTTP/1.1\r\n").unwrap();
    let resp_404 = handle_request(&req_404);
    let t3 = resp_404.status_code == 404;
    println!("TEST_404|{}|status={}", if t3 { "PASS" } else { "FAIL" }, resp_404.status_code);

    let resp_str = resp1.to_string();
    let t4 = resp_str.contains("HTTP/1.1 200") && resp_str.contains("Content-Length");
    println!("TEST_format|{}|{}", if t4 { "PASS" } else { "FAIL" }, &resp_str[..50]);
}
