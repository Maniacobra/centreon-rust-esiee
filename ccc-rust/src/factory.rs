pub mod common {
    tonic::include_proto!("com.centreon.common");
}

pub mod broker {
    tonic::include_proto!("com.centreon.broker");
}

use broker::*;

pub fn extract_value(input: &str) -> Option<(String, String)> {
    let start_index = input.find('{');
    let end_index = input.find('}');
    
    match (start_index, end_index) {
        (Some(start), Some(end)) if start < end => {
            Some((input[..start].to_string(), input[start..=end].to_string()))
        }
        _ => None,
    }
}

pub fn get_generic_name_or_index(j_data: serde_json::Value) -> Option<GenericNameOrIndex>
{
    let v_str = j_data.get("str");
    let v_idx = j_data.get("idx");

    let msg = GenericNameOrIndex {
        name_or_index: match (v_str, v_idx) {
            (None, Some(v)) => Some(generic_name_or_index::NameOrIndex::Idx(v_idx.unwrap().as_u64().unwrap())),
            (Some(v), None) => Some(generic_name_or_index::NameOrIndex::Str(v_idx.unwrap().as_str().unwrap().to_string())),
            _ => return None
        }
    };
    Some(msg)
}

pub fn get_generic_string(j_data: serde_json::Value) -> Option<GenericString>
{
    let v_str = j_data.get("str");

    let msg = GenericString {
        str_arg: match v_str {
            Some(v) => v.as_str().unwrap().to_string(),
            None => return None
        }
    };
    Some(msg)
}

pub fn get_sql_manager_stats_options(j_data: serde_json::Value) -> Option<SqlManagerStatsOptions>
{
    todo!()
}

pub fn get_sql_connection(j_data: serde_json::Value) -> Option<SqlConnection>
{
    todo!()
}

pub fn get_index_ids(j_data: serde_json::Value) -> Option<IndexIds>
{
    todo!()
}

pub fn get_ba_info(j_data: serde_json::Value) -> Option<BaInfo>
{
    todo!()
}

pub fn get_to_remove(j_data: serde_json::Value) -> Option<ToRemove>
{
    todo!()
}

pub fn get_log_level(j_data: serde_json::Value) -> Option<LogLevel>
{
    todo!()
}

pub fn get_log_flush_period(j_data: serde_json::Value) -> Option<LogFlushPeriod>
{
    todo!()
}

/*
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

#[derive(Debug, serde::Deserialize)]
struct MyObject {
    // Define your JSON structure fields here
    str_arg: String,
    // Add other fields as needed
}

#[tokio::main]
pub async fn send_message_get_log_info(
    pid: u32,
    method: &str,
    json_params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    /*/
    let json_str = r#"{"str_arg":"hellowrold"}"#;

    // Deserialize the JSON string into an instance of MyObject
    let my_object: MyObject = serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    // Access the fields of the deserialized object
    println!("str_arg: {}", my_object.str_arg);
    println!("test");
    */
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;

    let msg = broker::GenericString {
        // str_arg: my_object.str_arg.to_string(),
        str_arg: "".to_string(),
    };

    let request = tonic::Request::new(msg);

    let response = client.get_log_info(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}
*/