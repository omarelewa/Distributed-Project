use std::fs::File; // import File module from fs library to allow for creating files to log stats.

use std::io::Write; // import Write trait from io library to allow for writing to files. 

use std::net::{SocketAddr, SocketAddrV4, UdpSocket}; // import UdpSocket, SocketAddr, and SocketAddrV4 modules from net module to create socket and address for receiver and sender to send requests and receive acknowledgements from sender

use std::str::FromStr; // import FromStr module from str module to allow for converting strings to SocketAddr

use std::sync::mpsc::{channel, Receiver, Sender}; // import Sender, Receiver, and channel modules from mpsc module to allow for sending and receiving messages between threads

use std::sync::{Arc, Mutex}; // import Arc and Mutex modules from sync module to allow for sharing data between threads

use std::thread; // import thread module to allow for spawning threads

use std::thread::JoinHandle; // import JoinHandle module from thread module to allow for joining threads

use std::time::Duration; // import Duration module from time module to allow for sleeping // import Write module from io module to allow for writing to files

//use rand::prelude::{random};

pub struct RequestReceiver {
    // struct to hold receiver info
    addr: String,                                        // local address
    socket: Arc<Mutex<UdpSocket>>,                       // socket to receive requests
    sender: Arc<Mutex<Sender<([u8; 100], SocketAddr)>>>, // sender to send requests to sender thread
    receiver: Receiver<([u8; 100], SocketAddr)>, // receiver to receive requests from sender thread
    request_socket: UdpSocket,                   // socket to send requests
    load: Arc<Mutex<u16>>,                       // load to hold number of requests received
}

impl RequestReceiver {
    // impl RequestReceiver
    pub fn new(addr: String) -> RequestReceiver {
        // constructor for RequestReceiver struct to create new RequestReceiver object with given address and socket to receive requests and send acknowledgements to sender thread and receiver to receive requests from sender thread and socket to send requests to sender thread and load to hold number of requests received and return RequestReceiver object

        let mut request_addr = SocketAddrV4::from_str(&addr).unwrap(); // convert addr to SocketAddrV4

        request_addr.set_port(request_addr.port() + 100); // set request_addr port to addr port + 100 to get port for request_socket to send requests to sender thread on
        
        let (sender, receiver) = channel::<([u8; 100], SocketAddr)>(); // create channel to send and receive requests from sender thread and store sender and receiver in sender and receiver variables respectively
        
        RequestReceiver {
            // return RequestReceiver 
            addr: addr.clone(), // set addr to addr passed in to constructor to get local address for receiver to receive requests and send acknowledgements to sender thread on 

            socket: Arc::new(Mutex::new( // set socket to UdpSocket with local address passed in to constructor to receive requests and send acknowledgements to sender thread on 
                
                UdpSocket::bind(addr).expect("couldn't bind sender to address"), // bind socket to local address passed in to constructor

            )), // bind socket to addr

            sender: Arc::new(Mutex::new(sender)), // set sender to sender to send requests to sender thread 
            
            receiver: receiver, // set receiver to receiver to receive requests from sender thread
            
            request_socket: UdpSocket::bind(request_addr).expect("Failed to bind to request addr"), // bind request_socket to request_addr to send requests to sender thread on

            load: Arc::new(Mutex::new(0)), // set load to 0 to hold number of requests received

        } // return RequestReceiver

    } // end constructor

    pub fn listen(&self) -> JoinHandle<()> {
        // listen function to listen for requests and send them to sender thread to send requests and receive acknowledgements from sender thread
        
        println!("Listening on port {}", self.addr); // print listening message to console
        
        let load_arc = self.load.clone(); // clone load to load_arc in order to share load between threads
        
        let socket_arc = self.socket.clone(); // clone socket to socket_arc in order to share socket between threads
        
        let sender_arc = self.sender.clone(); // clone sender to sender_arc in order to share sender between threads
        
        return thread::spawn(move || loop {
            // spawn thread to listen for requests and send them to sender thread to send requests and receive acknowledgements from sender thread
        
            let socket = socket_arc.lock().unwrap(); // lock socket_arc
        
            let sender = sender_arc.lock().unwrap(); // lock sender_arc
        
            let mut buf = [0; 100]; // create buffer to hold message from sender thread
        
            let (_, src_addr) = (*socket).recv_from(&mut buf).expect("Didn't receive data"); // receive message from sender thread

            // let buf_str = str::from_utf8(&buf[..]).unwrap();
            // println!("{}", buf_str);
            // println!("Got a message from {}", src_addr);

            if buf[0] == 2
            // asking for load
            {
                let reply_addr = src_addr; // clone src_addr to reply_addr to send reply to sender thread

                let message = load_arc.lock().unwrap(); // lock load_arc

                // println!("Load: {}",*message);

                let message = (*message).to_be_bytes(); // convert load to bytes to send to sender thread

                // println!("{:?}", message);

                (*socket)
                    .send_to(&message, reply_addr)
                    .expect("Failed to send acknowledgement"); // send load to src_addr to sender thread
            } 
            else if buf[0] == 1 {}// do nothing
            
            else {
                // got a request from the client in buf
                
                let mut load = load_arc.lock().unwrap(); // lock load_arc
                
                *load += 1; // increment load by 1 to keep track of number of requests received from sender thread and store in load variable
                
                drop(load); // drop load variable to unlock load_arc for other threads to use load_arc
                
                sender
                    .send((buf, src_addr)) // send request and src_addr to sender thread
                    .expect("Failed to pass request in queue"); // add request to queue to be handled by sender thread
            } // end else
        }); // end spawn
    } // end listen

    pub fn log_stats(&self) -> JoinHandle<()> { // log_stats function to log stats to file and return JoinHandle to join thread to main thread when finished logging stats to file 
        
        let load_arc = self.load.clone(); // clone load to load_arc in order to share load between threads 

        let mut file = File::create({ // create file to log stats to and store in file variable 

            // create file to log stats
            let fname = self.addr.clone(); // clone addr to fname to create file name
            
            format!("{}.txt", fname.replace(":", "-")) // return file name with .txt extension
        }) // create file to log stats to and store in file variable 
        .unwrap(); // unwrap file creation to handle errors

        return thread::spawn(move || loop { // spawn thread to log stats to file and return JoinHandle to join thread to main thread when finished logging stats to file 

            // spawn thread to log stats to file

            let load_val = {

                // get load value
                let load = load_arc.lock().unwrap(); // lock load_arc
                
                *load // return load value
            }; // end get load value

            if let Err(_) = writeln!(file, "{}", load_val)
            // write load value to file
            {
                println!("Failed to write to file");
            } // end if let

            thread::sleep(Duration::from_secs(1)); // sleep for 1 second
            {
                () // do nothing
            } // end if let Err

            thread::sleep(Duration::from_secs(1)); // sleep for 1 second
        }); // end spawn
    } // end log_stats function

    pub fn handle_requests(&self) { 
        // handle_requests function to handle requests from sender thread

        loop {
            // loop to handle requests from sender thread

            // read request from queue, send a reply, then sleep
            let (request, addr) = self.receiver.recv().unwrap(); // get request from queue

            let mut load = self.load.lock().unwrap(); // lock load

            *load -= 1; // decrement load

            drop(load); // dropping mutex early as we no longer need it

            println!("{}", String::from_utf8(request.to_vec()).unwrap()); // print request to console
            
            // let reply = String::from("REQ PROCESSED");

            self.request_socket // send acknowledgement to sender thread
                .send_to(&request, addr) // send acknowledgement to sender thread
                .expect("Failed to send processed request"); // send request to client as reply to request from client 

            thread::sleep(Duration::from_micros(1000)); // sleep for 1 millisecond
        } // end loop
    } // end handle_requests
} // end impl RequestReceiver to handle requests from sender thread
