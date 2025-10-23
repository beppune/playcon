use std::ffi::OsStr;
use std::io::Write;

use interprocess::local_socket::{Listener, *};
use interprocess::local_socket::traits::Listener as Listen;

use std::io::Result;

enum Request {
    Log(String),
    Write(Stream, String),
}

impl Request {
    fn log(s:&str) -> Option<Request> {
        Some( Request::Log(String::from(s)) )
    }

    fn write(stream: Stream, buf: String) -> Option<Request> {
        Some( Request::Write(stream, buf) )
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
                Request::Write(mut stream, buf) => {
                    stream.write_all(buf.as_bytes()).unwrap();
                },
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
            Ok(stream) => {
                Request::write(stream, String::from("Client says bye!"))
            },
            Err(err) => Request::log(err.to_string().as_str()),
        }
    });

    loop {
        reactor.dispatch();
    }

    // wait for event (block)
    // dispatch event to proper handler
    // eventually enqueue other handlers

}
