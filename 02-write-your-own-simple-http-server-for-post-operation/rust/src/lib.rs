use itertools::Itertools;

use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

#[derive(Debug, Default)]
pub struct Response {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

#[derive(Default)]
pub struct ServerBuilder {
    addr: String,
    routes: Arc<Mutex<BTreeMap<String, Box<dyn 'static + Fn(Request) -> Response + Send + Sync>>>>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn addr(mut self, addr: &str) -> Self {
        self.addr = addr.to_string();
        self
    }

    pub async fn post(
        self,
        route: &str,
        handler: (impl 'static + Fn(Request) -> Response + Send + Sync),
    ) -> Self {
        let uri = format!("POST {}", route);
        println!("handler: {}", &uri);
        let routes = self.routes.clone();
        routes.lock().await.insert(uri, Box::new(handler));
        self
    }

    pub async fn get(
        self,
        route: &str,
        handler: (impl 'static + Fn(Request) -> Response + Send + Sync),
    ) -> Self {
        let uri = format!("GET {}", route);
        println!("handler: {}", &uri);
        let routes = self.routes.clone();
        routes.lock().await.insert(uri, Box::new(handler));
        self
    }

    pub async fn listen(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Listening on {}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let routes = self.routes.clone();

            tokio::spawn(async move {
                let mut stream = BufReader::new(stream);

                let mut status_line = String::new();
                stream.read_line(&mut status_line).await.unwrap();
                let Some((method, path)) = status_line
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .take(2)
                    .collect_tuple::<(String, String)>() else {
                        return;
                    };

                let handler_key = format!("{} {}", method, path);

                println!("request: {:?}", handler_key);

                let mut headers = BTreeMap::new();
                loop {
                    let mut line = String::new();
                    stream.read_line(&mut line).await.unwrap();
                    if line == "\r\n" || line == "\r" {
                        break;
                    }
                    let mut parts = line.split(": ");
                    headers.insert(
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap().to_string(),
                    );
                }

                let length = headers
                    .get("Content-Length")
                    .or(Some(&"0".to_string()))
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap();

                let mut body = vec![0; length];
                stream.read_exact(&mut body).await.unwrap();

                let req = Request {
                    method,
                    path,
                    headers,
                    body: String::from_utf8(body).unwrap(),
                };

                let routes = routes.lock().await;
                let route = routes.get(&handler_key);
                let res = match route {
                    Some(handler) => handler(req),
                    None => Response {
                        status: 404,
                        headers: Default::default(),
                        body: "Not Found".to_string(),
                    },
                };
                println!("resp {:?}", res);

                let canonical_status_info = match res.status {
                    200 => "OK",
                    404 => "Not Found",
                    _ => "Unknown",
                };

                let write_status = stream
                    .write_all(
                        format!(
                            "HTTP/1.1 {} {}\r\n\r\n{}",
                            res.status, canonical_status_info, res.body
                        )
                        .as_bytes(),
                    )
                    .await;

                if let Err(e) = write_status {
                    println!("failed to write to socket; err = {:?}", e);
                }
            });
        }
    }
}
