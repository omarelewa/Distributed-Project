
use std::fs::{File}; // import File module
// use  std::sync::mpsc::Sender;
use std::net::{UdpSocket, SocketAddr, SocketAddrV4}; // import UdpSocket, SocketAddr, and SocketAddrV4 modules from net module to create socket and address for receiver and sender 
use std::thread::JoinHandle; // import JoinHandle module from thread module to allow for joining threads
use std::time::Duration; // import Duration module from time module to allow for sleeping
use std::{thread}; // import thread module to allow for spawning threads
use std::sync::{Arc, Mutex}; // import Arc and Mutex modules from sync module to allow for sharing data between threads
use std::sync::mpsc::{ Sender, Receiver, channel}; // import Sender, Receiver, and channel modules from mpsc module to allow for sending and receiving messages between threads
use std::str::FromStr; // import FromStr module from str module to allow for converting strings to SocketAddr
use std::io::Write; // import Write module from io module to allow for writing to files
//use rand::prelude::{random};

pub struct RequestReceiver{ // struct to hold receiver info
    addr: String, // local address
    socket: Arc<Mutex<UdpSocket>>, // socket to receive requests
    sender: Arc<Mutex<Sender<([u8; 100], SocketAddr)>>>,  // sender to send requests to sender thread
    receiver:Receiver<([u8; 100], SocketAddr)>, // receiver to receive requests from sender thread
    request_socket: UdpSocket, // socket to send requests
    load: Arc<Mutex<u16>>, // load to hold number of requests received
}

impl RequestReceiver{ // impl RequestReceiver
    pub fn new(addr: String) -> RequestReceiver { // constructor
        let mut request_addr = SocketAddrV4::from_str(&addr).unwrap(); // convert addr to SocketAddrV4 
        request_addr.set_port(request_addr.port()+100);  // set request_addr port to addr port + 100
        let (sender, receiver) = channel::<([u8; 100], SocketAddr)>(); // create channel to send and receive requests
        RequestReceiver{ // return RequestReceiver
            addr: addr.clone(), // set addr 
            socket: Arc::new(Mutex::new(UdpSocket::bind(addr).expect("couldn't bind sender to address"))), // bind socket to addr
            sender: Arc::new(Mutex::new(sender)),  // set sender
            receiver: receiver, // set receiver
            request_socket: UdpSocket::bind(request_addr).expect("Failed to bind to request addr"),  // bind request_socket to request_addr 
            load: Arc::new(Mutex::new(0)) // set load to 0
        } // return RequestReceiver
    } // end constructor

    pub fn listen(&self) -> JoinHandle<()>{ // listen function
        println!("Listening on port {}", self.addr); // print listening message
        let load_arc = self.load.clone(); // clone load to load_arc
        let socket_arc = self.socket.clone(); // clone socket to socket_arc
        let sender_arc  = self.sender.clone(); // clone sender to sender_arc
        return thread::spawn(move || loop{ // spawn thread
                let socket =  socket_arc.lock().unwrap(); // lock socket_arc
                let sender = sender_arc.lock().unwrap(); // lock sender_arc
                let mut buf = [0; 100];     // create buffer to hold message
                let (_, src_addr) = (*socket).recv_from(&mut buf).expect("Didn't receive data"); // receive message
                // let buf_str = str::from_utf8(&buf[..]).unwrap();
                // println!("{}", buf_str);
                // println!("Got a message from {}", src_addr);
                if buf[0] == 2  // asking for load 
                {
                    let reply_addr = src_addr; // clone src_addr to reply_addr
          
                    let message = load_arc.lock().unwrap(); // lock load_arc
                    // println!("Load: {}",*message);
                    let message = (*message).to_be_bytes(); // convert load to bytes

                    // println!("{:?}", message);
                    (*socket).send_to(&message, reply_addr).expect("Failed to send acknowledgement"); // send load to src_addr
                }
                else if  buf[0] == 1 {}

                else {  // got a request from the client
                    let mut load = load_arc.lock().unwrap(); // lock load_arc
                    *load += 1; // increment load
                    drop(load); // drop load
                    sender.send((buf, src_addr)).expect("Failed to pass request in queue"); // add request to queue to be handled by sender thread
                } // end else
        }); // end spawn
    } // end listen

    pub fn log_stats(&self) -> JoinHandle<()>{ // log_stats function
        let load_arc = self.load.clone(); // clone load to load_arc

        let mut file = File::create({ // create file to log stats
            let fname = self.addr.clone(); // clone addr to fname 
            
            format!("{}.txt", fname.replace(":", "-")) // return file name with .txt extension
        }).unwrap(); // unwrap file

        return thread::spawn(move || loop{ // spawn thread to log stats
            let load_val = { // get load value 
                let load   = load_arc.lock().unwrap(); // lock load_arc 
                *load  // return load 
            }; // end get load value

    
           if let Err(_) =  writeln!(file, "{}", load_val) 
           {
             () // do nothing
           } // end if let Err
           thread::sleep(Duration::from_secs(1));  // sleep for 1 second
        }); // end spawn
    } // end log_stats function

    pub fn handle_requests(&self){ // handle_requests function to handle requests
        loop{
            // read request from queue, send a reply, then sleep
            let (request, addr) = self.receiver.recv().unwrap(); // get request from queue 
            let mut load = self.load.lock().unwrap(); // lock load 
            *load -= 1;  // decrement load
            drop(load);  // dropping mutex early as we no longer need it 
            println!("{}", String::from_utf8(request.to_vec()).unwrap()); // print request to console 
            // let reply = String::from("REQ PROCESSED");
            self.request_socket.send_to(&request, addr).expect("Failed to send processed request"); // send request to client 
            thread::sleep(Duration::from_micros(1000)); // sleep for 1 millisecond
        } // end loop
    } // end handle_requests
} // end impl RequestReceiver