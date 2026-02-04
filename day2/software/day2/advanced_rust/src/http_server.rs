use std::net::TcpListener;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::collections::HashMap;

pub struct HttpServer {
    pub host: String,
    pub port: u16,
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}
#[derive(Debug, Clone, Copy)]
pub enum HttpStatus {
    OK = 200,
    NotFound = 404,
    InternalServerError = 500,
}

pub struct HttpResponse {
    pub status_code: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub path : String,
}

impl HttpServer {
    pub fn new(host: String, port: u16) -> Self {HttpServer { host, port }}

    pub fn start(&self) -> std::io::Result<()> {
        let adress = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&adress)?;
        println!("Server running on http://{}", adress);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        let mut buffer = [0; 512];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                let response = "HTTP/1.1 200 OK\r\n\r\nCouCou";
                                stream.write_all(response.as_bytes()).unwrap();
                            }
                            Err(e) => {
                                eprintln!("Failed to read from connection: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
        Ok(()) 
    }

    pub fn parse_request(request: &str) -> Result<HttpRequest, String> {
        let lines: Vec<&str> = request.split("\r\n").collect();
        if lines.is_empty() {
            return Err("Empty request".to_string());
        }
        
        let request_line = lines[0];
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 { return Err("Invalid HTTP request line".to_string()); }
        let method = match parts[0]{
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => return Err("Unsupported HTTP method".to_string()),
        }; 

        let mut headers = HashMap::new();
        for line in &lines[1..] {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.to_string(), value.trim().to_string());
            }
        }
        let path = parts[1].to_string();
        Ok(HttpRequest {
            method,
            headers,
            path,
        })}
    
    pub fn handle_root(&self , _request : &HttpRequest) -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><body><h1>Bonjour</h1><p>C'est tout.</p></body>".to_string()
    }

    pub fn handle_not_found(&self , _request : &HttpRequest) -> String {
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><body><h1>404 Not Found</h1><p>The requested resource was not found.</p></body>".to_string()
    }

    pub fn handle_about(&self , _request : &HttpRequest) -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><body><h1>À propos de moi</h1></body>".to_string()
    }

    pub fn handle_api_data(&self , _request : &HttpRequest) -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"data\": \"Voici quelques données API.\"}".to_string()
    }

    pub fn route_request(&self , request : &HttpRequest) -> String {
        match request.path.as_str() {
            "/" => self.handle_root(request),
            "/about" => self.handle_about(request),
            "/api/data" => self.handle_api_data(request),
            _ => self.handle_not_found(request),
        }
    }
}

impl HttpResponse {
    pub fn generate(&self) -> String {
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code as u16, match self.status_code {
            HttpStatus::OK => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
        });
        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }
        format!("{}{}\r\n{}", status_line, headers, self.body)
    }
}