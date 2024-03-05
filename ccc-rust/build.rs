// build.rs

fn main() {
    prost_build::Config::new()
    .out_dir("src/proto") // Output directory for the generated code
    .compile_protos(&["src/test.proto"], &["src"]) // Paths to the proto files and their include directories
    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}
