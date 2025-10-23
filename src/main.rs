use std::ffi::OsStr;
use std::io::Write;

use interprocess::local_socket::{Listener, *};
use interprocess::local_socket::traits::Listener as Listen;

enum Request {
    Accept,
    Read,
}

struct Reactor {
    q:Vec<Request>
}

impl Reactor {
    fn new() -> Self {
        Self { 
            q: vec![],
        }
    }

    fn accept<F>(&mut self, ls:&mut Listener, hn:F)
        where F: Fn(Stream) -> Option<Request>
    {

        match ls.accept() {
            Ok(stream) => {
                if let Some(req) = hn(stream) {
                    self.q.push( req );
                }
            },
            Err(_) => todo!(),
        }

    }

}

fn main() {

    let option = ListenerOptions::new()
        .nonblocking(ListenerNonblockingMode::Stream)
        .name( OsStr::new("ThePipe").to_ns_name::<GenericNamespaced>().unwrap() );
    let mut pipe = option.create_sync().unwrap();
    
    let mut reactor = Reactor::new();
    reactor.accept(&mut pipe, |mut stream:Stream|{
        println!("Client accepted");
        stream.write_all(b"Hello\n").unwrap();
        None
    });

    // wait for event (block)
    // dispatch event to proper handler
    // eventually enqueue other handlers

}
