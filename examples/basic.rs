extern crate ccll;
use ccll::Cancellable;
use std::{net, io::{self, prelude::*}};

struct Service(net::TcpListener);

impl ccll::Cancellable for Service {
    type Error = io::Error; 
    fn for_each(&mut self) -> Result<ccll::LoopState, Self::Error> {
        let mut stream = self.0.accept()?.0;
        write!(stream, "hello!")?;
        Ok(ccll::LoopState::Continue)
    }
}

impl Service {
    fn new() -> Self {
        let listener = net::TcpListener::bind("127.0.0.1:12345").unwrap();
        Service(listener)
    }
}

fn main() {
    let mut s = Service::new();
    s.run();
}