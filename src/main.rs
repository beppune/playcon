use std::collections::VecDeque;
use std::ffi::OsStr;
use std::io::{ErrorKind, Write};

use interprocess::local_socket::{Listener, *};
use interprocess::local_socket::traits::Listener as Listen;

use std::io::Result;

type AcceptHandler = Box<dyn Fn(Stream)>;

enum Handler {
    OnAccept(AcceptHandler),
}

enum Event {
    Accept(Stream),
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
                    for handler in &self.handlers {
                        match handler {
                            Handler::OnAccept(callback) => {
                                // further get return value
                                // to enque
                                callback(stream);
                                break;
                            },
                        }
                    }
                },
            }
        }
    }

    fn accept<T>(&mut self, handler:T)
        where T: Fn(Stream) + 'static
    {
        self.handlers.push( Handler::OnAccept(Box::new(handler)) );
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
            reactor.accept( |mut stream| {
                stream.write_all(b"Ciaone!").unwrap();
            });
            // wait for event (block)
            // dispatch event to proper handler
            // eventually enqueue other handlers

            reactor.run();
}
