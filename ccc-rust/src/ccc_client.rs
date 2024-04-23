pub mod common {
    tonic::include_proto!("com.centreon.common");
}

pub mod broker {
    tonic::include_proto!("com.centreon.broker");
}

use broker::broker_client::BrokerClient;

pub async fn get_version() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = BrokerClient::connect("http://[::1]:51001").await?;

    let request = tonic::Request::new(());

    let response = client.get_version(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}


#[derive(Debug)]
struct MyStruct {
    // Define your struct fields here
    name: String,
    age: u32,
}




    
