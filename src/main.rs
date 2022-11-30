
// to run as server:
// cargo run <server_address> <client_address> 0 
// cargo run 127.0.0.1:7878 127.0.0.1:5656 0

// to run as client
// cargo run <client_address> <server_address> 1
// cargo run 127.0.0.1:5656 127.0.0.1:7878 1


mod receiver; // import receiver module from receiver.rs file 
mod sender; // import sender module from sender.rs file
use std::thread; // import thread module from std library to allow for spawning threads
use std::env; // import env module from std library to allow for getting command line arguments
use receiver::RequestReceiver; // import RequestReceiver struct from receiver module to allow for creating RequestReceiver objects
use sender::RequestSender; // import RequestSender struct from sender module to allow for creating RequestSender objects
fn main() {
    let args: Vec<String> = env::args().collect();  // get local and remote addresses from command line arguments 
    let send_addr = &args[1]; // get local address from first argument 
    let receive_addr = &args[2]; // get remote address from second argument
    let remote = &args[3]; // get remote flag (0 if server, 1 if client) from third argument 
    
    let receiver = RequestReceiver::new(receive_addr.to_string()); // create receiver with receive_addr as address 
    let sender = RequestSender::new(send_addr.to_string(), remote.to_string()); // create sender with send_addr as address 

    let t1 = thread::spawn( // spawn thread to listen for requests and send acknowledgements to sender 
        move || { // move receiver into thread  
            receiver.listen() // call listen function on receiver 
        } // end move 
    ); // end spawn 

    let t2 =thread::spawn( // spawn thread to send requests and receive acknowledgements from receiver 
        move || { // move sender into thread 
            sender.run() // call run function on sender
        } // end move 
    ); // end spawn

    t1.join().unwrap(); // join t1 thread to main thread 
    t2.join().unwrap(); // join t2 thread to main thread
    
}
