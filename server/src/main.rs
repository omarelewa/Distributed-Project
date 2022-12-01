mod receiver; // import receiver module from receiver.rs file to allow for creating RequestReceiver objects

use std::env; // import env module from std library to allow for getting command line arguments

use receiver::RequestReceiver; // import RequestReceiver from receiver module to allow for creating RequestReceiver objects

use std::process::exit; // import exit function from process module to allow for exiting the program with a status code when an error occurs or the program is finished running

use std::time::Duration;
// import Duration module from time module to allow for sleeping // import Write module from io module to allow for writing to files

use std::thread;
// import thread module to allow for spawning threads

fn main() {
    let args: Vec<String> = env::args().collect(); 
    // get server address from command line

    let receive_addr = &args[1]; 
    // local address is first argument

    println!("Server address: {}", receive_addr);

    let receiver = RequestReceiver::new(receive_addr.to_string()); 
    // create receiver with receive_addr as address

    loop {
        let t1 = receiver.listen(); 
        // listen for requests and send acknowledgements to sender
    
        let t2 = receiver.log_stats(); 
        // log stats every 10 seconds to stats.txt file
    
        let t3 = receiver.election();
    
        // let t4 = receiver.receive_index();
    
        receiver.handle_requests(); 
        // handle requests and send responses
    
        t1.join().unwrap(); 
        // join listen thread to main thread to keep program running until listen thread is finished
    
        t2.join().unwrap(); 
        // join log_stats thread to main thread to keep program running until log_stats thread is finished 
    
        t3.join().unwrap();
        // join send_index thread to main thread to keep program running until send_index thread is finished

        println!("Server is down");

        // sleep for 1 minute to allow for other servers to start up
        thread::sleep(Duration::from_secs(60));
    }

    // println!("Server shutting down"); // print message to console
    
    // program will exit when main thread ends
    // kill program with ctrl-c
    // or with kill command

    // exit program
    // exit(0);

}
