pub mod common {
    tonic::include_proto!("com.centreon.common");
}

pub mod broker {
    tonic::include_proto!("com.centreon.broker");
}

use broker::broker_client::BrokerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = BrokerClient::connect("http://[::1]:51001").await?;

    let request = tonic::Request::new(());

    let response = client.get_version(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}
