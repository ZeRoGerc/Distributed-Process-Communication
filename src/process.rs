use server::ProcessServer;
use client::ProcessClient;
use utils::*;
use std::sync::RwLock;
use iron::Listening;

lazy_static! {
    pub static ref CLOCK: RwLock<LamportClock> = RwLock::new(LamportClock::new());
}

pub fn incrementAndGetTime() -> u32 {
    let mut temp = CLOCK.write().unwrap();
    temp.incrementAndGet()
}

pub fn applyAndIncrementTime(other_time: u32) -> u32 {
    let mut temp = CLOCK.write().unwrap();
    temp.applyAndIncrement(other_time)
}

pub struct Process {
    provider: ProcessInfoProvider,
    server: ProcessServer,
    client: ProcessClient
}

impl Process {
    pub fn new(id: u32) -> Process {
        let provider = ProcessInfoProvider::new();
        let (server, client) = initServerAndClient(&provider, id);

        Process{
            provider: provider, 
            server: server, 
            client: client
        }
    }

    pub fn startProcess(&self) -> Listening {
        self.server.start()
    }

    pub fn sendMessage(&self, id: u32, msg: String) {
        let info: &ProcessInfo = match self.provider.getById(id) {
            Some(result) => result,
            None => {
                println!("Could not send message");
                return;
            },
        };
        
        self.client.sendMessage(info.ip.as_str(), info.port, msg.as_str())
    }
}

fn initServerAndClient(provider: &ProcessInfoProvider, id: u32) -> (ProcessServer, ProcessClient) {
    let info: &ProcessInfo = match provider.getById(id) {
        Some(result) => result,
        None => panic!("Could not find cfg for given id"),
    };
        
    let server = ProcessServer::new(info.ip.as_str(), info.port);
    let client = ProcessClient::new(id);
    (server, client)
}