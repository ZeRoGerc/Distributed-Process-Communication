use server::ProcessServer;
use client::ProcessClient;
use utils::*;
use std::sync::RwLock;
use iron::Listening;

lazy_static! {
    pub static ref CLOCK: RwLock<LamportClock> = RwLock::new(LamportClock::new());
}

pub fn increment_and_get_time() -> u32 {
    let mut temp = CLOCK.write().unwrap();
    temp.increment_and_get()
}

pub fn apply_and_increment_time(other_time: u32) -> u32 {
    let mut temp = CLOCK.write().unwrap();
    temp.apply_and_increment(other_time)
}

pub struct Process {
    provider: ProcessInfoProvider,
    server: ProcessServer,
    client: ProcessClient
}

impl Process {
    pub fn new(id: u32) -> Process {
        let provider = ProcessInfoProvider::new();
        let (server, client) = init_server_and_client(&provider, id);

        Process{
            provider: provider, 
            server: server, 
            client: client
        }
    }

    pub fn start_process(&self) -> Listening {
        self.server.start()
    }

    pub fn send_message(&self, id: u32, msg: String) {
        let info: &ProcessInfo = match self.provider.get_by_id(id) {
            Some(result) => result,
            None => {
                println!("Could not send message");
                return;
            },
        };
        
        self.client.send_message(info.ip.as_str(), info.port, msg.as_str())
    }
}

fn init_server_and_client(provider: &ProcessInfoProvider, id: u32) -> (ProcessServer, ProcessClient) {
    let info: &ProcessInfo = match provider.get_by_id(id) {
        Some(result) => result,
        None => panic!("Could not find cfg for given id"),
    };
        
    let server = ProcessServer::new(info.ip.as_str(), info.port);
    let client = ProcessClient::new(id);
    (server, client)
}