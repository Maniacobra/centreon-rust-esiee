pub mod common {
    tonic::include_proto!("com.centreon.common");
}

pub mod broker {
    tonic::include_proto!("com.centreon.broker");
}

use broker::broker_client::BrokerClient;
use broker::*;
use prost::Message;

#[tokio::main]
pub async fn send_message(pid: u32, method: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;
    let request = tonic::Request::new(());

    macro_rules! send_request {
        ($function:ident) => {{
            let response = client.$function(request).await?;
            let msg_response = response.into_inner();
            println!("{:?}", msg_response);
        }};
    }

    match method {
        "GetVersion" => send_request!(get_version),
        "GetProcessStats" => send_request!(get_process_stats),
        "GetGenericStats" => send_request!(get_generic_stats),
        "GetConflictManagerStats" => send_request!(get_conflict_manager_stats),
        "GetNumModules" => send_request!(get_num_modules),
        "GetNumEndpoint" => send_request!(get_num_endpoint),
        "GetProcessingStats" => send_request!(get_processing_stats),

        _ => {
            eprintln!("Unknown method '{}'", method)
        }
    }

    Ok(())
}

/////////////////////////////// FONCTION DE TEST
#[derive(Clone, PartialEq, ::prost::Message)]

pub struct SqlConnection {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
}

#[tokio::main]
pub async fn send_message_with_params(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = GenericNameOrIndex {
        name_or_index: Some(generic_name_or_index::NameOrIndex::Idx((0))),
    };

    let request = tonic::Request::new(msg);

    let response = client.get_modules_stats(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}

#[tokio::main]
pub async fn send_message_with_sql_params(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = broker::SqlConnection { id: Some(1) };

    let request = tonic::Request::new(msg);

    let response = client.get_sql_manager_stats(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}
