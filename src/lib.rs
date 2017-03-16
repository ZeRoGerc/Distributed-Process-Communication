extern crate iron;
extern crate rustc_serialize;
extern crate router;
extern crate rand;
extern crate logger;
extern crate hyper;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod server;
pub mod client;
pub mod utils;
pub mod process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
