use broker::broker_client::BrokerClient;
use broker::*;

use crate::factory::*;

#[tokio::main]
pub async fn send_message(pid: u32, method: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;
    let request = tonic::Request::new(());

    macro_rules! send_request {
        ($function:ident) => {{
            let response = client.$function(request).await?;
            let msg_response = response.into_inner();
            println!("\n---- RESPONSE ----\n");
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
            eprintln!("Unknown method '{}' or it needs data.", method)
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn send_message_with_data(pid: u32, method: &str, j_data: serde_json::Value) -> Result<(), Box<dyn std::error::Error>>
{
    let mut client = BrokerClient::connect(format!("http://[::1]:{}", pid)).await?;
    let request = tonic::Request::new(());

    macro_rules! send_request {
        ($function:ident, $data_type:ident) => {{
            let msg_opt = $data_type(j_data);
            if msg_opt == None {
                eprintln!("Invalid JSON for method {}", method);
                return Ok(());
            }
            let request = tonic::Request::new(msg_opt.unwrap());
            let response = client.$function(request).await?;
            let msg_response = response.into_inner();
            println!("\n---- RESPONSE ----\n");
            println!("{:?}", msg_response);
        }};
    }
    
    match method {
        "SetSqlManagerStats" => send_request!(set_sql_manager_stats, get_sql_manager_stats_options),
        "GetSqlManagerStats" => send_request!(get_sql_manager_stats, get_sql_connection),
        "GetMuxerStats" => send_request!(get_muxer_stats, get_generic_string),
        "GetModulesStats" => send_request!(get_modules_stats, get_generic_name_or_index),
        "GetEndpointStats" => send_request!(get_endpoint_stats, get_generic_name_or_index),
        "RebuildRRDGraphs" => send_request!(rebuild_rrd_graphs, get_index_ids),
        "GetBa" => send_request!(get_ba, get_ba_info),
        "RemoveGraphs" => send_request!(remove_graphs, get_to_remove),
        "RemovePollers" => send_request!(remove_poller, get_generic_name_or_index),
        "GetLogInfo" => send_request!(get_log_info, get_generic_string),
        "SetLogLevel" => send_request!(set_log_level, get_log_level),
        "SetLogFlushPeriod" => send_request!(set_log_flush_period, get_log_flush_period),
        _ => {
            eprintln!("Unknown method '{}' or it takes no data.", method)
        }
    }

    Ok(())
}