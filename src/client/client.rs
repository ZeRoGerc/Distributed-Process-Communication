extern crate hyper;

use rustc_serialize::json;
use hyper::*;
use std::io::Read;
use utils::*;
use process::*;

pub struct ProcessClient {
    pub id: u32
}

impl ProcessClient {
    pub fn new(id: u32) -> ProcessClient {
        ProcessClient{ id : id }
    }

    pub fn send_message(&self, ip: &str, port: u16, message: &str) {
        let request = JsonRequest {
            id : self.id,
            time : increment_and_get_time(),
            msg : message.to_string()
        };

        let client = Client::new();
        let mut res = client.post(format!("http://{}:{}/", ip, port).as_str())
            .body(json::encode(&request).unwrap().as_str())
            .send()
            .unwrap();

        assert_eq!(res.status, hyper::Ok);
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
    }
}