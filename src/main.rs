
// to run as server:
// cargo run <server_address> <client_address> 0 
// cargo run 127.0.0.1:7878 127.0.0.1:5656 0

// to run as client
// cargo run <client_address> <server_address> 1
// cargo run 127.0.0.1:5656 127.0.0.1:7878 1


mod receiver; // import receiver module
mod sender; // import sender module
use std::thread; // import thread module
use std::env; // import env module
use receiver::RequestReceiver; // import RequestReceiver struct
use sender::RequestSender; // import RequestSender struct
fn main() {
    let args: Vec<String> = env::args().collect();  // get local and remote addresses
    let send_addr = &args[1]; // get local address
    let receive_addr = &args[2]; // get remote address
    let remote = &args[3]; // get remote flag (0 if server, 1 if client)
    
    let receiver = RequestReceiver::new(receive_addr.to_string()); // create receiver with receive_addr as address
    let sender = RequestSender::new(send_addr.to_string(), remote.to_string()); // create sender with send_addr as address

    let t1 = thread::spawn( // spawn thread to listen for requests
        move || { // move receiver into thread
            receiver.listen() // call listen function
        } // end move
    ); // end spawn

    let t2 =thread::spawn( // spawn thread to send requests
        move || { // move sender into thread
            sender.run() // call run function
        } // end move
    ); // end spawn

    t1.join().unwrap(); // join t1 thread to main thread 
    t2.join().unwrap(); // join t2 thread to main thread
    
}