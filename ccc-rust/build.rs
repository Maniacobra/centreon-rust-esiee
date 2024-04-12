use std::{env, path::PathBuf};
use prost_build::Config;
use std::mem;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("process_stat_descriptor.bin"))
        .compile(&["proto/process_stat.proto"], &["proto"])
        .unwrap();

    let config = mem::take(Config::default().protoc_arg("--experimental_allow_proto3_optional"));
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("broker_descriptor.bin"))
        .compile_with_config(
            config,
            &["proto/broker.proto"],
            &["proto"])
        .unwrap();
}