extern crate iron;
extern crate rustc_serialize;
extern crate router;
extern crate rand;
extern crate logger;
extern crate hyper;

pub mod server;
pub mod client;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
