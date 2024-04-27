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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlConnection {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyMessage {
    #[prost(message, optional, tag = "1")]
    pub generic_name_or_index: ::core::option::Option<GenericNameOrIndex>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyBaInfo {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub output_file: String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericString {
    #[prost(string, tag = "1")]
    pub str_arg: String,
}

#[tokio::main]
pub async fn send_message_get_module_stats(
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
pub async fn send_message_get_sql_manager_stats(
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

#[tokio::main]
pub async fn send_message_get_muxer_stats(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = broker::GenericString {
        str_arg: "".to_string(),
    };

    let request = tonic::Request::new(msg);

    let response = client.get_muxer_stats(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}

#[tokio::main]
pub async fn send_message_getEndPointStats(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = GenericNameOrIndex {
        name_or_index: Some(generic_name_or_index::NameOrIndex::Idx((0))),
    };

    let request = tonic::Request::new(msg);

    let response = client.get_endpoint_stats(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}

#[tokio::main]
pub async fn send_message_get_Ba(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = broker::BaInfo {
        id: 0,
        output_file: "".to_string(),
    };

    let request = tonic::Request::new(msg);

    let response = client.get_ba(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}

#[tokio::main]
pub async fn send_message_get_log_info(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = broker::GenericString {
        str_arg: "".to_string(),
    };

    let request = tonic::Request::new(msg);

    let response = client.get_log_info(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}

/*
send_message_get_module_stats(0, "GetModulesStats", "").await?;
send_message_get_sql_manager_stats(0, "GetSqlManagerStats", "").await?;
send_message_get_muxer_stats(0, "GetMuxerStats", "").await?;
send_message_getEndPointStats(0, "GetEndpointStats", "").await?;
send_message_get_Ba(0, "GetBa", "").await?;
send_message_get_log_info(0, "GetLogInfo", "").await?;
*/
