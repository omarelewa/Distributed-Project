
mod sender; // import sender module to allow for sending requests to server 
// use std::thread;
use std::{env, fs}; // import env and fs modules to allow for getting command line arguments and creating files 
// use std::io;
use std::net::{SocketAddrV4}; // import SocketAddrV4 module from net module to create socket and address for receiver and sender
use std::str::FromStr; // import FromStr module from str module to allow for converting strings to SocketAddr 
use std::sync::{Arc, Mutex}; // import Arc and Mutex modules from sync module to allow for creating shared variables between threads
use std::thread; // import thread module to allow for spawning threads 
// use std::time::Duration;
// use receiver::RequestReceiver;
use sender::{RequestSender, ClientStats}; // import RequestSender and ClientStats from sender module to allow for sending requests to server and getting client stats


fn main() { 
    let args: Vec<String> = env::args().collect();  // get server address
    let send_addr = &args[1]; // local address

    let mut threads = vec![]; // vector to hold all threads

    let stats_arc = {
      let s: Vec<ClientStats> = vec![]; // vector to hold client stats
      Arc::new(Mutex::new(s))  // create shared variable to hold client stats
    }; // Arc to hold all client stats

    
    for i in 0..500{ // loop to create 500 threads to send requests to server 
        let mut send_addr = SocketAddrV4::from_str(send_addr).unwrap(); // convert to SocketAddrV4 type 

        send_addr.set_port(send_addr.port()+i); // set port to local port + i to create unique port for each thread 

        let mut rec_addr = send_addr.clone(); // clone send_addr to rec_addr to create unique port for receiver 

        rec_addr.set_port(rec_addr.port()+1000); // set rec_addr port to send_addr port + 1000 to create unique port for receiver

        let stats_arc_ = stats_arc.clone(); // clone stats_arc to stats_arc_ to allow for sharing stats_arc between threads

        let loop_fun = move || { // closure to be run in thread  
          let mut sender = RequestSender::new(send_addr.to_string(), rec_addr.to_string(), format!("Client {}", i)); // create sender with send_addr as address, rec_addr as receiver address, and format!("Client {}", i) as client name
          sender.init(); // initialize sender 
            for j in 0..1000{ // loop to send 1000 requests to server 

                sender.send(String::from(format!("Hello from {} [{}]", send_addr.to_string(), j))); // send message to server 
                // thread::sleep(Duration::from_secs(1));
            }
          let mut stats_vec = stats_arc_.lock().unwrap(); // lock stats_arc to get vector of client stats 
          stats_vec.push(sender.get_stats()); // push sender stats to stats_vec  
        }; // end of loop_fun closure

        threads.push(thread::spawn(move || loop_fun())); // spawn thread with loop_fun closure and push to threads vector 
    } // end of for loop to create 500 threads
 
    for t in threads{ // loop to join all threads to the main thread
      t.join().unwrap(); // join all threads
    }

    let stats_vec = stats_arc.lock().unwrap(); // lock stats_arc
    let stats_string = serde_json::to_string_pretty(&*stats_vec).unwrap(); // convert stats_vec to json string
    fs::write("stats.json", stats_string).unwrap(); // write stats_string to stats.json
}



