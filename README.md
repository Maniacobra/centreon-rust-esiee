ccc-Rust
===
Attempt to remake `centreon-collect-client` in Rust language.
This implementation is experimental and doesn't support reflection.

**How to use**

Start by cloning this repo and installing prost.
Do `cargo run -- -h` to get the help of the program.

Specify the port with `-p`
Ex : `cargo run -- -p 51001`
Then follow with `-c` for the command. Use braces if the methods takes data.
Examples :
```
cargo run -- -p 51001 -c GetVersion
cargo run -- -p 51001 -c 'GetModulesStats{"idx":1}'
```