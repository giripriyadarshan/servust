use tonic::transport::Endpoint;

mod proto;

use proto::hello::{greeter_client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = Endpoint::from_static("http://0.0.0.0:50051");

    let mut client = GreeterClient::connect(addr).await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
