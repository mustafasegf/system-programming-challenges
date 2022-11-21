use rust::ServerBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ServerBuilder::new()
        .addr("localhost:8080")
        .get("/", |req| {
            println!("{:?}", req);
            rust::Response {
                status: 200,
                headers: Default::default(),
                body: "Hello, world!".to_string(),
            }
        }).await
        .post("/", |req| {
            rust::Response {
                status: 200,
                headers: Default::default(),
                body: base64::encode(req.body).to_string(),
            }
        }).await
        .listen()
        .await
}
