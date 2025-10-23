use std::error::Error;
use std::ffi::OsStr;
use std::io::{Error as IoError, Write};

use interprocess::local_socket::{Listener, *};
use interprocess::local_socket::traits::Listener as Listen;

use std::io::Result;

enum Request {
    Log(String)
}

impl Request {
    fn log(s:&str) -> Option<Request> {
        Some( Request::Log(String::from(s)) )
    }
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
        where F: Fn(Result<Stream>) -> Option<Request>
    {

        match ls.accept() {
            Ok(stream) => {
                if let Some(req) = hn( Ok(stream) ) {
                    self.q.push( req );
                }
            },
            err => {
                if let Some(req) = hn( err ) {
                    self.q.push( req );
                }
            },
        }

    }

    fn dispatch(&mut self) {

        if let Some(req) = self.q.pop() {
            match req {
                Request::Log(buf) => println!("Log: {buf}"),
            }
        }

    }

}

fn main() {

    let option = ListenerOptions::new()
        .nonblocking(ListenerNonblockingMode::Stream)
        .name( OsStr::new("ThePipe").to_ns_name::<GenericNamespaced>().unwrap() );
    let mut pipe = option.create_sync().unwrap();

    let mut reactor = Reactor::new();
    reactor.accept(&mut pipe, |res:Result<Stream>|{
        match res {
            Ok(mut stream) => {
                stream.write_all( b"Ciaone\n" ).unwrap();
                Request::log("Client says bye!")
            },
            Err(err) => Request::log(err.description()),
        }
    });

    reactor.dispatch();

    // wait for event (block)
    // dispatch event to proper handler
    // eventually enqueue other handlers

}
