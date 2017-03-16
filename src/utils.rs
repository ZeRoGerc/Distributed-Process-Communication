use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::max;
use std::collections::HashMap;
use regex::Regex;

#[derive(RustcDecodable, RustcEncodable)]
pub struct JsonRequest {
  pub id: u32,
  pub time: u32,
  pub msg: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct JsonResponse {
  pub response: String
}

pub struct LamportClock {
  time: u32
}

impl LamportClock {
  pub fn new() -> LamportClock {
    LamportClock { time: 0 }
  }

  pub fn incrementAndGet(&mut self) -> u32 {
    self.time += 1;
    self.time
  }

  pub fn applyAndIncrement(&mut self, other_time: u32) -> u32 {
    self.time = max(self.time, other_time) + 1;
    self.time
  }
}

pub struct ProcessInfo {
  pub ip: String,
  pub port: u16
}

pub struct ProcessInfoProvider {
  process_map: HashMap<u32, ProcessInfo>,
  delimiters: Regex
}

impl ProcessInfoProvider {
  pub fn getById(&self, id: u32) -> Option<&ProcessInfo> {
    self.process_map.get(&id)
  }

  pub fn new() -> ProcessInfoProvider {
    let mut provider = ProcessInfoProvider{ 
      process_map: HashMap::new(),
      delimiters: Regex::new(r"[:=]").unwrap()
    };
    fill_from_file(&mut provider);
    provider
  }
}

fn fill_from_file(provider: &mut ProcessInfoProvider ) {
  let f = File::open("process.cfg").expect("Could not open cfg file process.cfg");
  let mut reader = BufReader::new(f);

  let lines = reader.lines();
  for (id, line) in lines.enumerate() {
    let s = line.unwrap();
    let parts: Vec<&str> = provider.delimiters.split(&s).collect();

    let port: u16 = parts[2].trim()
      .parse()
      .expect("Could not read from file process.cfg");

    provider.process_map.insert(
      (id + 1) as u32, 
      ProcessInfo{ ip : parts[1].to_string(), port: port }
    );
  }

  print_cfg(provider);
}

fn print_cfg(provider: &mut ProcessInfoProvider) {
  for id in provider.process_map.keys() {
    match provider.process_map.get(id) {
      Some(info) => {
        println!("process_id: {}, ip: {:?}, port: {}", id, info.ip, info.port);
      },
      None => {},
    }
  }
}