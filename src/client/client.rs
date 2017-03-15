extern crate hyper;

use hyper::*;
use std::io::Read;

pub struct ProcessClient {
}

impl ProcessClient {
    
    pub fn sendMessage(&self, port: u16, message: String) {
        let client = Client::new();
        let mut res = client.get(format!("http://localhost:{}/", port).as_str())
            .send()
            .unwrap();

        assert_eq!(res.status, hyper::Ok);
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        println!("{}", s);
    }
}