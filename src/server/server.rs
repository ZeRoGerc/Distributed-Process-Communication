extern crate rand;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::Listening;
use rand::Rng;
use rustc_serialize::json;
use router::Router;
use utils::JsonResponse;

pub struct ProcessServer {
    pub port: u16,
}

impl ProcessServer {
    pub fn start(&self) -> Listening {
        let mut router = Router::new();
        router.get("/", handler, "index");

        Iron::new(router)
          .http(("localhost", self.port))
          .expect("Unable to start server")
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
  let response = JsonResponse { response: pick_response("Ulad".to_string()) };
  let out = json::encode(&response).unwrap();

  let content_type = "application/json".parse::<Mime>().unwrap();
  Ok(Response::with((content_type, status::Ok, out)))
}

fn pick_response(name: String) -> String {
  let num = rand::thread_rng().gen_range(1, 4);

  let response = match num {
    1 => format!("Hello {}!", name),
    2 => format!("Did you see that ludicrous display last night, {}?", name),
    3 => format!("Nice weather for ducks, isn't it {}", name),
    _ => format!("")     // match is exhaustive
  };

  response.to_string()
}