use tonic::transport::Channel;
use tonic::Request;
use tonic_reflection::pb::server_reflection_client::ServerReflectionClient;
use tonic_reflection::pb::ServerReflectionRequest;
use futures_util::stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = ServerReflectionClient::new(Channel::from_static("http://localhost:6463").connect().await?);

    let request = ServerReflectionRequest {
        // Set the desired fields of the request
        host: "localhost".into(),
        message_request: Some(tonic_reflection::pb::server_reflection_request::MessageRequest::ListServices("".into())),
    };

    let request_stream = stream::iter(vec![request]);
    let response = client.server_reflection_info(Request::new(request_stream)).await?;

    // response is a Stream of ServerReflectionResponse messages
    let mut inbound = response.into_inner();

    // Process each response from the server (assuming there might be more than one for illustration)
    while let Some(response) = inbound.message().await? {
        // Process the response here
        println!("{:?}", response);
    }

    Ok(())
}

/*

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the gRPC server address
    let address = "http://[::1]:50051".parse()?;
    
    // Create a client connected to the server address
    let mut client = ServerReflectionClient::new(Channel::from_static(address).connect().await?);

    // Create a ServerReflectionRequest
    let request = ServerReflectionRequest {
        // Set the desired fields of the request
        host: "localhost".into(),
        message_request: Some(your_service_definition::server_reflection_request::MessageRequest::ListServices("".into())),
    };

    // Make the request using the client
    let response = client.server_reflection_info(Request::new(vec![request].into_iter())).await?;

    // Process the response stream
    let mut inbound = response.into_inner();
    while let Some(response) = inbound.message().await? {
        // Process each ServerReflectionResponse
        println!("{:?}", response);
    }

    Ok(())
}


*/