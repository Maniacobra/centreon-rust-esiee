#![allow(warnings, unused)]

extern crate colored;
extern crate getopts;
use colored::*;
use getopts::Options;
use std::env;
use std::process::ExitCode;
use tokio::runtime::Runtime;

mod ccc_client;
use ccc_client::send_message;
use ccc_client::send_message_getEndPointStats;
use ccc_client::send_message_get_Ba;
use ccc_client::send_message_get_log_info;
use ccc_client::send_message_get_module_stats;
use ccc_client::send_message_get_muxer_stats;
use ccc_client::send_message_get_sql_manager_stats;

fn long_options() -> Options {
    let mut opts = Options::new();
    opts.optopt(
        "p",
        "port",
        "Specifies the gRPC server port to connect to.",
        "NUMBER",
    );
    opts.optopt("c", "command", "Specifies the command to use", "COMMAND");
    opts.optflag(
        "h",
        "help",
        "Displays a general help or a help message on the command.",
    );
    opts.optflag("l", "list", "Displays the available methods.");
    opts.optflag("v", "version", "Displays the version of ccc.");
    opts.optflag(
        "n",
        "nocolor",
        "Outputs are displayed with the current color.",
    );
    opts
}

fn usage(opts: Options, program: String, color_enabled: bool) {
    if color_enabled {
        let brief = format!("{}: {} [options]", "Usage".blue(), program);
        println!("'ccc' uses centreon-broker or centreon-engine gRPC api to communicate with them");
        print!("{}", opts.usage(&brief));
        println!("\n{}:\n ccc -p 51001 --list", "Examples".blue());
        println!(" # Lists available functions from gRPC interface at port 51000");
        println!(" ccc -p 51001 GetVersion .");
        println!(" # Calls the GetVersion method.");
    } else {
        let brief = format!("Usage: {} [options]", program);
        println!("'ccc' uses centreon-broker or centreon-engine gRPC api to communicate with them");
        print!("{}", opts.usage(&brief));
        println!("\nExamples:\n ccc -p 51001 --list");
        println!(" # Lists available functions from gRPC interface at port 51000");
        println!(" ccc -p 51001 GetVersion .");
        println!(" # Calls the GetVersion method.");
    }
}

use prost_serde::build_with_serde;

#[allow(dead_code)]
fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut port: u32 = 0;

    let mut list: bool = false;
    let mut help: bool = false;
    let mut color_enabled: bool = true;
    let mut command: String = String::new();

    let opts: Options = long_options();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("Unrecognized argument '{}'", f.to_string());
            panic!("error")
        }
    };
    if matches.opt_present("v") {
        println!("ccc");
    }
    if matches.opt_present("n") {
        color_enabled = false;
    }
    if matches.opt_present("l") {
        list = true;
    }
    if matches.opt_present("p") {
        port = matches.opt_get("p").unwrap().unwrap();
    }
    if matches.opt_present("c") {
        command = matches.opt_get("c").unwrap().unwrap();
    }
    if matches.opt_present("h") {
        help = true;
    }

    if help {
        usage(opts, program, color_enabled);
        return ExitCode::SUCCESS;
    }
    if port == 0 {
        // fix tempo car le port est pas recup
        eprintln!("You must specify a port for the connection to the gRPC server");
        return ExitCode::from(2);
    }

    // SENDING MESSAGE

    /*
            send_message(port,command.as_str());
            send_message_get_module_stats(0, "GetModulesStats", "");
            send_message_get_sql_manager_stats(0, "GetSqlManagerStats", "");
            send_message_get_muxer_stats(0, "GetMuxerStats", "");
            send_message_getEndPointStats(0, "GetEndpointStats", "");
            send_message_get_Ba(0, "GetBa", "");
            send_message_get_log_info(0, "GetLogInfo", "");
    */
    let result_send_message = send_message(port, command.as_str());
    match result_send_message {
        Ok(_) => (),
        Err(e) => {
            eprintln!("\n---- ERROR ----\n");
            eprintln!("{:?}", e);
            eprintln!();
            return ExitCode::from(1);
        }
    }
    /*
        let result_module_stats = send_message_get_module_stats(port, command.as_str(), "");
        match result_module_stats {
            Ok(_) => (),
            Err(e) => {
                eprintln!("\n---- ERROR ----\n");
                eprintln!("{:?}", e);
                eprintln!();
                return ExitCode::from(1);
            }
        }

        let result_sql_manager_stats = send_message_get_sql_manager_stats(port, command.as_str(), "");
        match result_sql_manager_stats {
            Ok(_) => (),
            Err(e) => {
                eprintln!("\n---- ERROR ----\n");
                eprintln!("{:?}", e);
                eprintln!();
                return ExitCode::from(1);
            }
        }

        let result_muxer_stats = send_message_get_muxer_stats(port, command.as_str(), "");
        match result_muxer_stats {
            Ok(_) => (),
            Err(e) => {
                eprintln!("\n---- ERROR ----\n");
                eprintln!("{:?}", e);
                eprintln!();
                return ExitCode::from(1);
            }
        }

        let result_endpoint_stats = send_message_getEndPointStats(port, command.as_str(), "");
        match result_endpoint_stats {
            Ok(_) => (),
            Err(e) => {
                eprintln!("\n---- ERROR ----\n");
                eprintln!("{:?}", e);
                eprintln!();
                return ExitCode::from(1);
            }
        }

        let result_ba = send_message_get_Ba(port, command.as_str(), "");
        match result_ba {
            Ok(_) => (),
            Err(e) => {
                eprintln!("\n---- ERROR ----\n");
                eprintln!("{:?}", e);
                eprintln!();
                return ExitCode::from(1);
            }
        }

        let result_log_info = send_message_get_log_info(port, command.as_str(), "");
        match result_log_info {
            Ok(_) => (),
            Err(e) => {
                eprintln!("\n---- ERROR ----\n");
                eprintln!("{:?}", e);
                eprintln!();
                return ExitCode::from(1);
            }
        }
    */
    let msg_fn = match command.as_str() {
        "module_stats" => {
            |pid, method, json_params| send_message_get_module_stats(pid, method, json_params);
        }
        "sql_manager_stats" => {
            |pid, method, json_params| send_message_get_sql_manager_stats(pid, method, json_params);
        }
        "muxer_stats" => {
            |pid, method, json_params| send_message_get_muxer_stats(pid, method, json_params);
        }
        "end_point_stats" => {
            |pid, method, json_params| send_message_getEndPointStats(pid, method, json_params);
        }
        "ba" => {
            |pid, method, json_params| send_message_get_Ba(pid, method, json_params);
        }
        "log_info" => {
            |pid, method, json_params| send_message_get_log_info(pid, method, json_params);
        }
        _ => {
            eprintln!("Unknown command msg_fn: {}", command);
            return ExitCode::from(1);
        }
    };
    ExitCode::SUCCESS
}
