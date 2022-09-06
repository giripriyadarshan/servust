mod db;

use db::pool;
use tonic::{transport::Server, Request, Response, Status};

mod lib;

use crate::lib::hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[derive(Debug, Default)]
pub struct MyGreeter<T> {
    _connection: Option<T>,
}

#[tonic::async_trait]
impl<T: std::marker::Sync + std::marker::Send + 'static> Greeter for MyGreeter<T> {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(GreeterServer::new(MyGreeter {
            _connection: Some(pool()),
        }))
        .serve(addr)
        .await?;

    Ok(())
}
