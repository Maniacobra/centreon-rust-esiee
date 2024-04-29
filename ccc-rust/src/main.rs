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

mod factory;
use factory::*;

use serde_json;

use crate::ccc_client::*;

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
        println!("'ccc-rust' uses centreon-broker or centreon-engine gRPC api to communicate with them");
        print!("{}", opts.usage(&brief));
        println!("\n{}:\n ccc-rust -p 51001 --list", "Examples".blue());
        println!(" # Lists available functions from gRPC interface at port 51001");
        println!(" ccc-rust -p 51001 -c GetVersion");
        println!(" # Calls the GetVersion method.");
        println!(" ccc-rust -p 51001 -c GetModulesStats{{\"idx\":2}}");
        println!(" # Calls the GetModulesStats method with data '2'.");
    } else {
        let brief = format!("Usage: {} [options]", program);
        println!("'ccc--rust' uses centreon-broker or centreon-engine gRPC api to communicate with them");
        print!("{}", opts.usage(&brief));
        println!("\nExamples:\n ccc--rust -p 51001 --list");
        println!(" # Lists available functions from gRPC interface at port 51001");
        println!(" ccc-rust -p 51001 -c GetVersion");
        println!(" # Calls the GetVersion method.");
        println!(" ccc-rust -p 51001 -c GetModulesStats{{\"idx\":2}}");
        println!(" # Calls the GetModulesStats method with data '2'.");
    }
}

fn display_list(color_enabled: bool) {
    if color_enabled {
        println!("{}:", "Available methods".green());
        println!("{} GetVersion", " ".repeat(4).green());
        println!("{} GetProcessStats", " ".repeat(4).green());
        println!("{} GetGenericStats", " ".repeat(4).green());
        println!("{} GetConflictManagerStats", " ".repeat(4).green());
        println!("{} GetNumModules", " ".repeat(4).green());
        println!("{} GetNumEndpoint", " ".repeat(4).green());
        println!("{} GetProcessingStats", " ".repeat(4).green());
        println!("{} SetSqlManagerStats", " ".repeat(4).green());
        println!("{} GetSqlManagerStats", " ".repeat(4).green());
        println!("{} GetMuxerStats", " ".repeat(4).green());
        println!("{} GetModulesStats", " ".repeat(4).green());
        println!("{} GetEndpointStats", " ".repeat(4).green());
        println!("{} RebuildRRDGraphs", " ".repeat(4).green());
        println!("{} GetBa", " ".repeat(4).green());
        println!("{} RemoveGraphs", " ".repeat(4).green());
        println!("{} RemovePollers", " ".repeat(4).green());
        println!("{} GetLogInfo", " ".repeat(4).green());
        println!("{} SetLogLevel", " ".repeat(4).green());
        println!("{} SetLogFlushPeriod", " ".repeat(4).green());
    } else {
        println!("Available methods:");
        println!(" GetVersion");
        println!(" GetProcessStats");
        println!(" GetGenericStats");
        println!(" GetConflictManagerStats");
        println!(" GetNumModules");
        println!(" GetNumEndpoint");
        println!(" GetProcessingStats");
        println!(" SetSqlManagerStats");
        println!(" GetSqlManagerStats");
        println!(" GetMuxerStats");
        println!(" GetModulesStats");
        println!(" GetEndpointStats");
        println!(" RebuildRRDGraphs");
        println!(" GetBa");
        println!(" RemoveGraphs");
        println!(" RemovePollers");
        println!(" GetLogInfo");
        println!(" SetLogLevel");
        println!(" SetLogFlushPeriod");
    }
}

#[allow(dead_code)]
fn main() -> ExitCode {

    ////////////// GET OPT

    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut port: u32 = 0;

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
        println!("ccc-rust_1.0");
        return ExitCode::SUCCESS;
    }
    if matches.opt_present("n") {
        color_enabled = false;
    }
    if matches.opt_present("l") {
        display_list(color_enabled);
        return ExitCode::SUCCESS;
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
        eprintln!("You must specify a port for the connection to the gRPC server");
        return ExitCode::from(1);
    }

    ////////////// JSON PARSING

    if command.len() == 0 {
        eprintln!("Missing command");
        return ExitCode::from(1);
    }

    let mut cmd_str = command.as_str();
    let mut opt_data: Option<serde_json::Value> = None;
    let mut temp = cmd_str.to_string();
    match extract_value(cmd_str) {
        Some((name, data)) => {
            temp = name;
            // Convert to JSON
            match serde_json::from_str(data.as_str()) {
                Ok(j) => opt_data = j,
                Err(..) => {
                    eprintln!("Invalid JSON syntax : {data}");
                    return ExitCode::from(1);
                }
            }
        },
        None => (),
    }
    cmd_str = temp.as_str();

    if opt_data == None && !cmd_str.chars().all(char::is_alphanumeric) {
        eprintln!("Invalid characters in command : {cmd_str}");
        return ExitCode::from(1);
    }

    ////////////// SENDING MESSAGE
    
    let result_send_message = match opt_data {

        None => send_message(port, cmd_str),

        Some(j_data) => send_message_with_data(port, cmd_str, j_data)

    };
    match result_send_message {
        Ok(_) => (),
        Err(e) => {
            eprintln!("\n---- ERROR ----\n");
            eprintln!("{:?}", e);
            eprintln!();
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS

}
