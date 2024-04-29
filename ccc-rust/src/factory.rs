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
            (Some(v), None) => Some(generic_name_or_index::NameOrIndex::Str(v_str.unwrap().as_str().unwrap().to_string())),
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

pub fn get_sql_manager_stats_options(j_data: serde_json::Value) -> Option<SqlManagerStatsOptions> {
    let slowest_statements_count = j_data.get("slowest_statements_count");
    let slowest_queries_count = j_data.get("slowest_queries_count");

    let msg = SqlManagerStatsOptions {
        slowest_statements_count: match slowest_statements_count {
            Some(v) => Some(v.as_u64().unwrap() as u32),
            None => return None,
        },
        slowest_queries_count: match slowest_queries_count {
            Some(v) => Some(v.as_u64().unwrap() as u32),
            None => return None,
        },
    };
    Some(msg)
}



pub fn get_sql_connection(j_data: serde_json::Value) -> Option<SqlConnection> {
    let v_str = j_data.get("str");

    let msg = SqlConnection {
        id: match v_str {
            Some(v) => Some(v.as_str().unwrap().parse::<u32>().unwrap()), 
            None => return None,
        },
    };

    Some(msg)
}

pub fn get_index_ids(j_data: serde_json::Value) -> Option<IndexIds> {
    let v_ids = j_data.get("ids");

    let msg = IndexIds {
        index_ids: match v_ids {
            Some(v) => v.as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect(),
            None => return None,
        },
    };

    Some(msg)
}

pub fn get_ba_info(j_data: serde_json::Value) -> Option<BaInfo>
{
    let v_id = j_data.get("id");
    let v_output_file = j_data.get("output_file");

    let msg = BaInfo {
        id: match v_id {
            Some(v) => v.as_u64().unwrap() as u64,
            None => return None,
        },
        output_file: match v_output_file {
            Some(v) => v.as_str().unwrap().to_string(),
            None => return None,
        },
    };

    Some(msg)
}

pub fn get_to_remove(j_data: serde_json::Value) -> Option<ToRemove> {
    let v_index_ids = j_data.get("index_ids");
    let v_metric_ids = j_data.get("metric_ids");

    let msg = ToRemove {
        index_ids: match v_index_ids {
            Some(v) => v.as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect(),
            None => return None,
        },
        metric_ids: match v_metric_ids {
            Some(v) => v.as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect(),
            None => return None,
        },
    };

    Some(msg)
}

pub fn get_log_level(j_data: serde_json::Value) -> Option<LogLevel> {
    let v_level = j_data.get("level");
    let v_logger = j_data.get("logger");

    let msg = LogLevel {
        level: match v_level {
            Some(v) => v.as_i64().unwrap() as i32, 
            None => return None,
        },
        logger: match v_logger {
            Some(v) => v.as_str().unwrap().to_string(), 
            None => return None,
        },
    };

    Some(msg)
}

pub fn get_log_flush_period(j_data: serde_json::Value) -> Option<LogFlushPeriod>
{
    let v_period = j_data.get("period");

    let msg = LogFlushPeriod {
        period: match v_period {
            Some(v) => v.as_u64().unwrap() as u32,
            None => return None,
        },
    };

    Some(msg)
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