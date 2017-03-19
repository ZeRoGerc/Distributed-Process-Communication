extern crate rand;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::Listening;
use rustc_serialize::json;
use router::Router;
use utils::JsonResponse;
use utils::JsonRequest;
use std::io::Read;
use process::*;

pub struct ProcessServer {
  pub ip: String,
  pub port: u16
}

impl ProcessServer {

  pub fn new(ip: &str, port: u16) -> ProcessServer {
    ProcessServer { 
      ip: ip.to_owned(),
      port: port
    }
  }

  pub fn start(&self) -> Listening {
    let mut router = Router::new();
    router.post("/", post_handler, "post");

    println!("Server is starting");

    Iron::new(router)
      .http((self.ip.as_str(), self.port))
      .expect("Unable to start server")
  }
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    req.body.read_to_string(&mut payload).unwrap();

    let request: JsonRequest = json::decode(payload.as_str()).unwrap();


    let time = apply_and_increment_time(request.time);

    println!("received from:{} msg:{} time:{}", request.id, request.msg, time);

    let response = JsonResponse { response: "ok".to_owned() };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
  }