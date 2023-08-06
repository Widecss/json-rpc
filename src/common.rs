use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub enum NetworkError {
    ServerError
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcRequest {
    method: String,
    args: HashMap<String, Value>,
    id: usize,
}

impl JsonRpcRequest {
    pub fn method(&self) -> &String {
        &self.method
    }
    pub fn get_arg_value(&self, name: &String) -> Option<&Value> {
        self.args.get(name)
    }
    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponse {
    result: Option<Value>,
    error: Option<String>,
    id: usize,
}

impl JsonRpcResponse {
    pub fn new(id: usize) -> Self {
        Self {
            result: None,
            error: None,
            id,
        }
    }
}

impl JsonRpcResponse {
    pub fn set_result(&mut self, return_value: Value) {
        self.result = Some(return_value);
    }
    pub fn set_error(&mut self, error_message: String) {
        self.error = Some(error_message);
    }
}

#[derive(PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    UNKNOWN,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::UNKNOWN
    }
}


pub struct HttpStatusCode(u16);

impl HttpStatusCode {
    pub fn get_phrase(&self) -> &'static str {
        match self.0 {
            200 => "OK",

            400 => "Bad Request",
            405 => "Method Not Allowed",

            500 => "Internal Server Error",
            505 => "HTTP Version Not Supported",

            _ => "Unknown",
        }
    }

    pub fn ok() -> Self {
        Self { 0: 200 }
    }
    pub fn bad_request() -> Self {
        Self { 0: 400 }
    }
    pub fn method_not_allowed() -> Self {
        Self { 0: 405 }
    }
    pub fn internal_server_error() -> Self {
        Self { 0: 500 }
    }
    pub fn http_version_not_supported() -> Self {
        Self { 0: 505 }
    }
}

#[derive(Default)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    http_version: String,

    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn push_state(&mut self, line: String) {
        let line = line.trim().to_string();
        let mut sp = line.split(" ");
        self.method = match sp.next() {
            Some(m) => {
                match m {
                    "GET" => HttpMethod::GET,
                    "POST" => HttpMethod::POST,
                    _ => HttpMethod::UNKNOWN
                }
            }
            None => { HttpMethod::UNKNOWN }
        };
        self.path = match sp.next() {
            Some(m) => { m.to_string() }
            None => { "".to_string() }
        };
        self.http_version = match sp.next() {
            Some(m) => { m.to_string() }
            None => { "".to_string() }
        };
    }

    pub fn push_headers(&mut self, line: String) {
        match line.split_once(": ") {
            None => {}
            Some((k, v)) => {
                self.headers.insert(k.to_string(), v.to_string());
            }
        }
    }
    pub fn push_body(&mut self, body: &mut Vec<u8>) {
        self.body.append(body);
    }

    pub fn method_is(&self, method: HttpMethod) -> bool {
        self.method == method
    }
    pub fn http_version(&self) -> &String {
        &self.http_version
    }
}

pub struct HttpResponse {
    pub http_version: String,
    pub status_code: HttpStatusCode,

    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            http_version: "HTTP/1.1".to_string(),
            status_code: HttpStatusCode::ok(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn write_body(&mut self, body: String) {
        self.body.push_str(body.as_str());
    }

    pub fn set_status(&mut self, sc: HttpStatusCode) {
        self.status_code = sc;
    }

    pub fn set_http_version(&mut self, hv: String) {
        self.http_version = hv;
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\n\r\n{}",
            self.http_version,
            self.status_code.0,
            self.status_code.get_phrase(),
            self.body
        )
    }
}

