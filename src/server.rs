use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::common::{HttpMethod, HttpRequest, HttpResponse, HttpStatusCode, JsonRpcRequest, JsonRpcResponse};
use crate::service::RpcService;

pub struct JsonRpcServer {
    service: Box<dyn RpcService>,
}

impl JsonRpcServer {
    pub fn new(sv: Box<dyn RpcService>) -> Self {
        Self { service: sv }
    }

    pub fn _thread(&self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            match stream.set_read_timeout(Some(Duration::new(0, 500))) {
                Ok(_) => {}
                Err(_) => {
                    let mut response = HttpResponse::new();
                    response.set_status(HttpStatusCode::internal_server_error());
                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    continue;
                }
            }

            let mut buf_reader = BufReader::new(&stream);

            // 第一行
            let mut first_line = String::new();
            let ret = buf_reader.read_line(&mut first_line);
            if ret.is_err() {
                continue;
            }

            // req
            let mut http_request = HttpRequest::default();
            http_request.push_state(first_line);

            // resp
            let mut http_response = HttpResponse::new();
            http_response.set_http_version(http_request.http_version().clone());

            if !http_request.method_is(HttpMethod::POST) {
                http_response.set_status(HttpStatusCode::method_not_allowed());
                stream.write_all(http_response.to_string().as_bytes()).unwrap();
                continue;
            }
            if !http_request.http_version().starts_with("HTTP/1") {
                http_response.set_status(HttpStatusCode::http_version_not_supported());
                stream.write_all(http_response.to_string().as_bytes()).unwrap();
                continue;
            }

            // 其他行
            loop {
                let mut header_line = String::new();
                match buf_reader.read_line(&mut header_line) {
                    Ok(_) => {
                        if header_line == "\r\n" {
                            break;
                        }
                        http_request.push_headers(header_line);
                    }
                    Err(_) => { break; }
                }
            }

            let body = &mut Vec::<u8>::new();
            let ret = buf_reader.read_to_end(body);
            if ret.is_err() {
                let err = ret.unwrap_err();
                if err.kind() != ErrorKind::TimedOut {
                    http_response.set_status(HttpStatusCode::bad_request());
                    stream.write_all(http_response.to_string().as_bytes()).unwrap();
                    continue;
                }
            }
            http_request.push_body(body);

            match String::from_utf8(http_request.body.clone()) {
                Ok(text) => {
                    match serde_json::from_str::<JsonRpcRequest>(text.as_str()) {
                        Ok(json_request) => {
                            let mut json_response = JsonRpcResponse::new(
                                json_request.id()
                            );
                            self.service.request(&json_request, &mut json_response);

                            match serde_json::to_string::<JsonRpcResponse>(&json_response) {
                                Ok(json_response_text) => {
                                    http_response.set_status(HttpStatusCode::ok());
                                    http_response.write_body(json_response_text);

                                    stream.write_all(http_response.to_string().as_bytes()).unwrap();
                                    continue;
                                }
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
            http_response.set_status(HttpStatusCode::bad_request());
            stream.write_all(http_response.to_string().as_bytes()).unwrap();
            continue;
        }
    }
    pub fn start(self, addr: SocketAddr) -> JoinHandle<()> {
        let sr = Arc::<Self>::new(self);
        thread::spawn(move || {
            sr._thread(addr);
        })
    }
}

unsafe impl Send for JsonRpcServer {}

unsafe impl Sync for JsonRpcServer {}