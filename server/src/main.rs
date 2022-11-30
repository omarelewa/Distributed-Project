
mod receiver;
use std::env;
use receiver::RequestReceiver;

fn main() {
    let args: Vec<String> = env::args().collect(); // get server address

    let receive_addr = &args[1]; // local address

    let receiver = RequestReceiver::new(receive_addr.to_string()); // create receiver

    let t1 = receiver.listen();  // listen for requests

    let t2 = receiver.log_stats(); // log stats

    receiver.handle_requests(); // handle requests

    t1.join().unwrap(); // join listen thread

    t2.join().unwrap(); // join log_stats thread
}