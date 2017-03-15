extern crate process_communication;

use process_communication::server::ProcessServer;
use process_communication::client::ProcessClient;
use std::env;
use std::io;

fn parse_and_send(input: String) {
    let port: u16 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid port");
            return;
        }
    };

    println!("Sending message on port: {}", port);

    let client = ProcessClient{};
    let message = String::new();
    client.sendMessage(port, message); 
}

fn main() {
    let mut port: u16 = 8080;
    for (num, argument) in env::args().enumerate() {
        if num == 0 { 
            continue; 
        }
        
        port = match argument.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("First argument must be a valid port");
                break;
            }
        }
    }

    println!("Server running on port: {}", port);

    let ser = ProcessServer {port : port};
    let listener = ser.start();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().is_empty() {
            println!("Server is stopped");
            break;
        }

        parse_and_send(input);
    }
}