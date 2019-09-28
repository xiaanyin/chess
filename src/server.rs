use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

use game::*;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_ip: String,
    pub server_port: i32,
}

pub struct Server {
    config: Config
}

impl Server {
    pub fn new(config_file: &str) -> Server {
        Server {
            config: {
                let mut file = File::open(config_file).expect("Unable to open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");
                serde_yaml::from_str(&contents).unwrap()
            }
        }
    }

    pub fn startup(&self) {
        let ip = format!("{}:{}", self.config.server_ip, self.config.server_port);
        let listener = TcpListener::bind(ip).unwrap();
        println!("listening started, ready to accept");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 100];
        stream.read(&mut buffer).unwrap();
        let message: Cow<str> = String::from_utf8_lossy(&buffer[..]);
        let chess_board: &str = message.trim_end_matches('\u{0}');

        let mut board: Board = Board::new();

        println!("[{}]", chess_board);
        board.init_board(chess_board);

        let response: String = board.search();
        println!("[{}]", response);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

