use std::collections::VecDeque;
use std::ffi::OsStr;
use std::io::{ErrorKind, Write};

use interprocess::local_socket::{Listener, *};
use interprocess::local_socket::traits::Listener as Listen;

use std::io::Result;
use std::io::Read;

type AcceptHandler = Box<dyn Fn(Stream) -> Option<Event>>;
type ReadHandler = Box<dyn Fn(usize) -> Option<Event>>;

enum Handler {
    OnAccept(AcceptHandler),
    OnRead(ReadHandler),
}

enum Event {
    Accept(Stream),
    Read(Stream,String),
}

impl Event {
    fn read(stream:Stream, string:String) -> Option<Event> {
        Some( Event::Read(stream, string) )
    }
}

struct Reactor {
    listener: Listener,
    queue: VecDeque<Event>,
    handlers: Vec<Handler>,
}

impl Reactor {
    fn new(listener: Listener) -> Self {
        Self {
            listener,
            queue: VecDeque::new(),
            handlers: vec![],
        }
    }

    fn run(&mut self) {

        if self.handlers.is_empty() {
            return;
        }

        loop {
            match self.listener.accept() {
                Ok(stream) => { 
                    self.queue.push_back( Event::Accept(stream) );
                    break;
                },
                Err(err) if err.kind() == ErrorKind::WouldBlock => {},
                Err(_) => {},
            }
        }

        // dispatch
        while let Some(event) = self.queue.pop_front() {
            match event {
                Event::Accept(stream) => {
                    if let Some(Handler::OnAccept(callback)) = &self.handlers.iter().find( |h| matches!(h, Handler::OnAccept(_)) ) {
                        if let Some(ev) = callback(stream) {   
                            self.queue.push_back( ev ); 
                        }
                    }
                },
                Event::Read(mut stream, mut string) => {
                    if let Some(Handler::OnRead(callback)) = &self.handlers.iter().find( |h| matches!(h, Handler::OnRead(_)) ) {
                        let ev:Option<Event>;
                        match stream.read_to_string(&mut string) {
                            Ok(amount) if amount == 0 => {
                                ev = Event::read(stream,string);
                            },
                            Ok(amount) => {
                                ev = callback(amount);
                            },
                            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                                ev = Event::read(stream, string);
                            },
                            Err(_) => {
                                ev = None;
                            }
                        }
                        if let Some(e) = ev {
                            self.queue.push_back( e );
                        }
                    }
                }
            }
        }
    }

    fn accept<T>(&mut self, handler:T)
        where T: Fn(Stream) -> Option<Event> + 'static
    {
        self.handlers.push( Handler::OnAccept(Box::new(handler)) );
    }

    fn read<T>(&mut self, handler:T)
        where T: Fn(usize) -> Option<Event> + 'static
    {
        self.handlers.push( Handler::OnRead(Box::new(handler)) );
    }

}

fn main() {

    // let mut queue:VecDeque<Event> = VecDeque::new();

    let option = ListenerOptions::new()
        .nonblocking(ListenerNonblockingMode::Stream)
        .name( OsStr::new("ThePipe")
            .to_ns_name::<GenericNamespaced>().unwrap() );
            let listener = option.create_sync().unwrap();

            let mut reactor = Reactor::new(listener);

            reactor.read( |amount| {
                println!("Red {amount}");
                None
            });

            reactor.accept( |mut stream| {
                stream.write_all(b"Ciaone!\n").unwrap();
                Event::read(stream, String::new())
            });
            // wait for event (block)
            // dispatch event to proper handler
            // eventually enqueue other handlers

            reactor.run();
}
