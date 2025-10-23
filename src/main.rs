use std::ffi::OsStr;

use interprocess::local_socket::{traits::Listener, *};

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

    fn accept<F>(&mut self, hn:F)
        where F: Fn() -> Option<Request>
    {

        let req = hn();

        match req {
            Some(r) => self.q.push( r ),
            None => {},
        }

    }

}

fn main() {

    let mut option = ListenerOptions::new()
        .nonblocking(ListenerNonblockingMode::Stream)
        .name( OsStr::new("ThePipe").to_ns_name::<GenericNamespaced>().unwrap() );
    let mut pipe = option.create_sync().unwrap();
    
    let mut reactor = Reactor::new();
    reactor.accept(||{
        println!("client accepted");
        None
    });

    pipe.accept().unwrap();


    // wait for event (block)
    // dispatch event to proper handler
    // eventually enqueue other handlers

}
