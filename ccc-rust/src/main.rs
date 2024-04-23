extern crate colored;
extern crate getopts;
use colored::*;
use getopts::Options;
use std::env;
use std::process::ExitCode;

mod ccc_client;
use ccc_client::*;

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
    opts.optopt("j", "json", "Specifies the json to use", "JSON");
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

#[macro_use]
extern crate serde_json;
use serde::Deserialize;

macro_rules! create_struct_from_string {
    ($struct_name:ident, $($field_name:ident : $field_value:expr),*) => {
        {
            $struct_name {
                $($field_name: $field_value),*
            }
        }
    };
}

#[derive(Debug, Deserialize)]
struct MyObject {
    id: u32,
    text: String,
    id2: u32,
    text2: String,
}

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

    if matches.opt_present("j") {
        let json: OneStruct = serde_json::from_str(&matches.opt_str("j").unwrap()).unwrap();
        // Access the fields of the deserialized object
        println!("id: {}", json.id);
        println!("text: {}", json.text);
        println!("id2: {}", json.id2);
        println!("text2: {}", json.text2);

        let mut macro_args = String::new();
        let mut field_values = String::new();

        let json_fields = json
            .into_iter()
            .collect::<Vec<(String, serde_json::Value)>>();

        for (field, value) in json_fields {
            macro_args.push_str(&format!("{}: {}, ", field, value));
            field_values.push_str(&format!("{}, ", value));
        }

        macro_args.pop(); // Remove trailing comma
        field_values.pop(); // Remove trailing comma

        let macro_code = format!("create_struct_from_string!(MyObject, {})", macro_args);
        let field_values_code = format!("create_struct_from_string!(MyObject, {})", field_values);

        println!("Macro code: {}", macro_code);
        println!("Field values code: {}", field_values_code);

        // Now you can use the macro code and field values code as needed
    }

    //get_version().await;

    let json_str = r#"{"id": 5, "text": "Hello World2","id2": 8, "text2": "Hello World2"}"#;

    // Deserialize the JSON string into an instance of MyObject
    let my_object: MyObject = serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    // Access the fields of the deserialized object
    println!("id: {}", my_object.id);
    println!("text: {}", my_object.text);
    println!("id2: {}", my_object.id2);
    println!("text2: {}", my_object.text2);

    let input = "MyObject"; // This could be any string

    // Create the struct instance using the macro
    let my_struct = match input {
        "MyObject" => {
            create_struct_from_string!(MyObject, id: 5, text: "John".to_string(), id2: 8, text2: "Doe".to_string())
        } // Add more cases for other struct types if needed
        _ => panic!("Unknown struct type"),
    };

    // Now you have an instance of the struct
    println!("{:?}", my_struct);

    return ExitCode::SUCCESS;
}
