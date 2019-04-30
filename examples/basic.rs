extern crate ccll;
use ccll::Cancellable;
use std::{net, time,io::{self, prelude::*}};
use std::thread;

struct Service(net::TcpListener);

impl ccll::Cancellable for Service {
    type Error = io::Error; 
    fn for_each(&mut self) -> Result<ccll::LoopState, Self::Error> {
        println!("in for each!");
        let mut stream = self.0.accept()?.0;
        writeln!(stream, "hello!")?;
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

    let h = s.spawn();
    let exit = h.canceller();

    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(10));
        exit.cancel();
    });
    h.wait();
}