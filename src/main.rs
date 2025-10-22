
trait EventSource {}

struct AcceptClient {}
impl AcceptClient {
    fn new() -> Self {
        Self {  }
    }
}
struct Reactor {}
impl Reactor {
    fn new(pipeserver: &u8) -> Self {
        Self {  }
    }

    fn enqueue(&self, es: AcceptClient, hn: impl Fn(&mut u8))  {
        todo!()
    }

    fn accept(&self, hn: impl Fn(&mut u8))  {
        todo!()
    }
}

impl EventSource for AcceptClient {}

fn main() {

    let pipeserver:u8 = 0;

    let mut reac = Reactor::new(&pipeserver);
    let es = AcceptClient::new();
    let mut hn = |pipe:&mut u8| {
        // write to pipe
    };
    reac.enqueue(es, hn);
    reac.accept(hn);
    reac.accept(|pipe:&mut u8|{
        //write to pipe
    });

    // Main loop
    //
    // register initial handler
    // wait for event (block)
    // dispatch event to proper handler
    // eventually enqueue other handlers

}
