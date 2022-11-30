
mod receiver; // import receiver module from receiver.rs file 
use std::env; // import env module from std library to allow for getting command line arguments
use receiver::RequestReceiver; // import RequestReceiver from receiver module to allow for creating RequestReceiver objects 

fn main() {
    let args: Vec<String> = env::args().collect(); // get server address from command line
    let receive_addr = &args[1]; // local address is first argument
    let receiver = RequestReceiver::new(receive_addr.to_string()); // create receiver with receive_addr as address
    let t1 = receiver.listen();  // listen for requests and send acknowledgements
    let t2 = receiver.log_stats(); // log stats every 10 seconds
    receiver.handle_requests(); // handle requests and send responses
    t1.join().unwrap(); // join listen thread to main thread
    t2.join().unwrap(); // join log_stats thread to main thread
}