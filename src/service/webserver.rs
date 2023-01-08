use crate::service::thread_pool::ThreadPool;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, sync::Arc,
};

type Application = Arc<dyn Fn(Request) -> Response + Send + Sync>;

pub fn run_server(application: Application)
    {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Received connection!");

        let app = Arc::clone(&application);
        pool.execute(|| {
            handle_connection(stream, app);
        });
    }
}

fn handle_connection(mut stream: TcpStream, application: Application)
    {
    let mut buf_reader = BufReader::new(&mut stream);
    let lines = buf_reader
        .by_ref()
        .lines()
        .map(|e| e.expect("Expected UTF-8 string"));
    let request: Vec<String> = lines.take_while(|line| !line.is_empty()).collect();

    println!("New request: {:#?}", &request);
    let parsed_req = parse_request(&request, &mut buf_reader);
    println!("Parsed request: {:#?}", parsed_req);

    let resp = application(parsed_req.unwrap());
    let length = resp.body.len();
    let response =
        format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", resp.status, resp.status_text, length, resp.body);

    stream.write_all(response.as_bytes()).unwrap();
}
// TODO do not drop connection after single request if Connection: keep-alive

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub uri: URI,
    pub headers: Headers,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub status: u64,
    pub status_text: String,
    pub headers: Headers,
    pub body: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    PlainText, // text/plain
    HTML,      // text/html
    JSON,      // application/json
    XML,       // application/xml
}

pub type URI = Vec<String>;
pub type Headers = HashMap<String, String>;

// TODO propagate errors into result instead of option
fn parse_request(
    req_lines: &Vec<String>,
    buffer: &mut BufReader<&mut TcpStream>,
) -> Option<Request> {
    let first_line: Vec<_> = req_lines.get(0)?.split(' ').collect();
    let method: Method = match *first_line.get(0)? {
        "GET" => Some(Method::GET),
        "POST" => Some(Method::POST),
        "PATCH" => Some(Method::PATCH),
        "DELETE" => Some(Method::DELETE),
        "PUT" => Some(Method::PUT),
        _ => None,
    }?;
    let uri: URI = first_line.get(1)?.split('/').map(String::from).collect();

    let headers: HashMap<String, String> = req_lines
        .iter()
        .skip(1)
        .map(|h| {
            let header: Vec<_> = h.split(": ").collect();
            let key = header.get(0);
            let value = header.get(1);
            match (key, value) {
                (Some(k), Some(v)) => Some((String::from(*k), String::from(*v))),
                _ => None,
            }
        })
        .flatten()
        .collect();

    let content_length = match headers.get("Content-Length") {
        Some(v) => v.parse::<usize>().ok(),
        None => None,
    };

    let body = match content_length {
        Some(l) => {
            let req_body: Vec<u8> = buffer.bytes().map(Result::unwrap).take(l).collect();
            String::from_utf8(req_body).ok()
        }
        None => None,
    };

    Some(Request {
        method,
        uri,
        headers,
        body,
    })
}
