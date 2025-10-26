use std::collections::VecDeque;
use std::ffi::OsStr;
use std::io::Write;

use interprocess::local_socket::{Listener, *};
use interprocess::local_socket::traits::Listener as Listen;

use std::io::Result;

type AcceptHandler = Box<(dyn Fn(Stream) -> Option<Event>)>;

enum Event {
    OnAccept(Stream, AcceptHandler),
}

fn main() {

    let mut queue:VecDeque<Event> = VecDeque::new();

    let option = ListenerOptions::new()
        .nonblocking(ListenerNonblockingMode::Stream)
        .name( OsStr::new("ThePipe")
            .to_ns_name::<GenericNamespaced>().unwrap() );
    let mut listener = option.create_sync().unwrap();

    match listener.accept() {
        Ok(stream) => {
            let ev = Event::OnAccept(stream, Box::new(|mut stream|{
                stream.write_all(b"Ciaone").unwrap();
                None
            }));
            queue.push_back( ev );
        },
        Err(err) => println!("{err}"),
    }

    match queue.pop_front() {
        Some(ev) => match ev {
            Event::OnAccept(stream, handler) => {
                if let Some(enq) = handler(stream) {
                    queue.push_back( enq );
                }
            },
        },
        None => {},
    } 
    // wait for event (block)
    // dispatch event to proper handler
    // eventually enqueue other handlers

}
