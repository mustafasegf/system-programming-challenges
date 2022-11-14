use tokio::io::{AsyncWriteExt, BufReader};
use tokio::{io::AsyncBufReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let ln = TcpListener::bind("localhost:8080").await?;
    println!("Listening on {}", ln.local_addr()?);
    loop {
        let (stream, addr) = ln.accept().await?;
        let mut stream = BufReader::new(stream);

        println!("connected to {}", addr);
        tokio::spawn(async move {
            let mut line = String::new();
            if let Err(..) = stream.read_line(&mut line).await {
                println!("error reading line");
                return;
            }
            let s: Vec<&str> = line.split_whitespace().collect();
            if s.len() < 3 {
                println!("invalid request");
                return;
            }
            let (method, path) = (s[0], s[1]);
            if method == "GET" {
                if let Err(..) = stream
                    .write_all(
                        format!(
                            "<HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html\r\n\
                    \r\n\
                    <p>anda mengakses {}</p>",
                            path
                        )
                        .as_bytes(),
                    )
                    .await
                {
                    println!("error writing response");
                    return;
                }
            } else {
                if let Err(..) = stream
                    .write_all(
                        b"<HTTP/1.1 405 Method Not Allowed\r\n\
                    \r\n",
                    )
                    .await
                {
                    println!("error writing response");
                    return;
                }
            }
        });
    }
}
