use crate::service::thread_pool::ThreadPool;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Received connection!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let lines = buf_reader
        .by_ref()
        .lines()
        .map(|e| e.expect("Expected UTF-8 string"));
    let request: Vec<String> = lines.take_while(|line| !line.is_empty()).collect();

    println!("New request: {:#?}", &request);
    let parsed_req = parse_request(&request);
    println!("Parsed request: {:#?}", parsed_req);
    let req_body: Vec<_> = buf_reader.bytes().map(Result::unwrap).take(18).collect();
    // let x: String = req_body.into();
    println!("Request body: {:?}", String::from_utf8(req_body));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
// TODO do not drop connection after single request if Connection: keep-alive

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub uri: URI,
    pub headers: Headers,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

pub type URI = Vec<String>;
pub type Headers = HashMap<String, String>;

fn parse_request(req_lines: &Vec<String>) -> Option<Request> {
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

    Some(Request {
        method,
        uri,
        headers,
    })
}
