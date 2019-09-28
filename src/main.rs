#![allow(unused_imports, dead_code, unused_mut, unused_variables)]

extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

mod game;
mod server;

fn main() {
    let server = server::Server::new("resources/server_init.yaml");
    server.startup();
}


