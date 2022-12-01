use std::fs::File;
// import File module from fs library to allow for creating files to log stats.

use std::io::Write;
// import Write trait from io library to allow for writing to files.

use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
// import UdpSocket, SocketAddr, and SocketAddrV4 modules from net module to create socket and address for receiver and sender to send requests and receive acknowledgements from sender

use std::str::FromStr;
// import FromStr module from str module to allow for converting strings to SocketAddr

use std::sync::mpsc::{channel, Receiver, Sender};
// import Sender, Receiver, and channel modules from mpsc module to allow for sending and receiving messages between threads

use std::sync::{Arc, Mutex};
// import Arc and Mutex modules from sync module to allow for sharing data between threads

use std::thread;
// import thread module to allow for spawning threads

use std::thread::JoinHandle;
// import JoinHandle module from thread module to allow for joining threads

use std::time::Duration;
// import Duration module from time module to allow for sleeping // import Write module from io module to allow for writing to files

use std::net::IpAddr;

use std::str;

use std::process::exit;

//use rand::prelude::{random};

// Ring Leader Election Function
//  - Takes in a list of all servers and the index of the current server
//  - Returns the index of the leader
//  - If the leader is the current server, returns the index of the current server
//  - If the leader is not the current server, returns the index of the leader
//  - If the leader is not in the list of servers, returns the index of the current server
//  - If the list of servers is empty, returns the index of the current server
//  - If the index of the current server is not in the list of servers, returns the index of the current server

pub struct RequestReceiver {
    // struct to hold receiver info
    addr: String,
    // local address of receiver to bind to socket to receive requests from sender and send acknowledgements to sender
    socket: Arc<Mutex<UdpSocket>>,
    // socket to receive requests from sender and send acknowledgements to sender
    sender: Arc<Mutex<Sender<([u8; 100], SocketAddr)>>>,
    // sender to send requests to sender thread using channel to send requests to sender thread in a thread safe manner through a mutex lock that is shared between threads
    receiver: Receiver<([u8; 100], SocketAddr)>,
    // receiver to receive requests from sender thread
    request_socket: UdpSocket,
    // socket to send requests
    election_addr: String,
    // socket to send election messages to other servers
    servers: Vec<SocketAddr>,
    // list of all servers in the ring
    index: usize,
    // index of current server in the list of servers
    load: Arc<Mutex<u16>>,
    // load to hold number of requests received
}

impl RequestReceiver {
    // impl RequestReceiver
    pub fn new(addr: String) -> RequestReceiver {
        // constructor for RequestReceiver struct to create new RequestReceiver object with given address and socket to receive requests and send acknowledgements to sender thread and receiver to receive requests from sender thread and socket to send requests to sender thread and load to hold number of requests received and return RequestReceiver object

        let mut request_addr = SocketAddrV4::from_str(&addr).unwrap();
        // convert addr to SocketAddrV4

        println!("request_addr: {}", request_addr);
        // print request_addr

        request_addr.set_port(request_addr.port() + 100);
        // set request_addr port to addr port + 100 to get port for request_socket to send requests to sender thread on

        println!("request_addr port: {}", request_addr.port());
        // print request_addr port

        let (sender, receiver) = channel::<([u8; 100], SocketAddr)>();
        // create channel to send and receive requests from sender thread and store sender and receiver in sender and receiver variables respectively

        println!("sender: {:?}", sender);
        // print sender

        println!("receiver: {:?}", receiver);
        // print receiver

        let mut i = 0; // initialize i to 0
        if addr == "127.0.0.1:5656" {
            // if addr is
            i = 0;
        } else if addr == "127.0.0.1:5657" {
            i = 1;
        } else if addr == "127.0.0.1:5658" {
            i = 2;
        }

        let mut elect_socket_port = "8080".to_string();
        if addr == "127.0.0.1:5656" {
            elect_socket_port = "9000".to_string();
        } else if addr == "127.0.0.1:5657" {
            elect_socket_port = "9001".to_string();
        } else if addr == "127.0.0.1:5658" {
            elect_socket_port = "9002".to_string();
        }

        // initialize servers list with all servers in the ring
        let servers = vec![
            SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:8080").unwrap()), // server 1
            SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:8081").unwrap()), // server 2
            SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:8082").unwrap()), // server 3
        ];

        RequestReceiver {
            // return RequestReceiver
            addr: addr.clone(),
            // set addr to addr passed in to constructor to get local address for receiver to receive requests and send acknowledgements to sender thread on

            // println!("addr: {}", addr);
            socket: Arc::new(Mutex::new(
                // set socket to UdpSocket with local address passed in to constructor to receive requests and send acknowledgements to sender thread on
                UdpSocket::bind(addr).expect("couldn't bind sender to address"),
                // bind socket to local address passed in to constructor
            )),
            // bind socket to address passed in to constructor
            sender: Arc::new(Mutex::new(sender)),
            // set sender to sender to send requests to sender thread using arc and mutex to share data between threads

            // println!("sender: {:?}", sender);
            receiver: receiver,

            // set receiver to receiver to receive requests from sender thread
            request_socket: UdpSocket::bind(request_addr).expect("Failed to bind to request addr"),

            // bind request_socket to request_addr to send requests to sender thread on
            election_addr: elect_socket_port, // set election_addr to elect_socket to send election messages to other servers on

            // bind election_socket to addr to send election messages to other servers on
            servers: servers,
            // initialize servers to empty vector
            index: i,
            // initialize index to 0 to get index of current server in the list of servers
            load: Arc::new(Mutex::new(0)),
            // set load to 0 to hold number of requests received

            // election_socket : UdpSocket::bind(SocketAddrV4::from_str(&addr).unwrap()).expect("Failed to bind to election addr"),
        }
        // return RequestReceiver object
    }
    // end constructor

    pub fn listen(&self) -> JoinHandle<()> {
        // listen function to listen for requests and send them to sender thread to send requests and receive acknowledgements from sender thread and return JoinHandle to join thread to main thread to allow for main thread to wait for thread to finish before exiting program and allow for thread to run in the background while main thread runs in the foreground

        // print index of current server in the list of servers
        println!("index: {}", self.index);

        // print election_addr to send election messages to other servers on
        println!("election_addr: {}", self.election_addr);

        // print servers list
        println!("servers: {:?}", self.servers);

        // print election_socket address
        // println!(
        //     "election_socket: {:?}",
        //     self.election_socket.local_addr().unwrap()
        // );

        println!("Listening on port {}", self.addr);
        // print listening message to console

        let load_arc = self.load.clone();
        // clone load to load_arc in order to share load between threads
        println!("load_arc: {:?}", load_arc);

        let socket_arc = self.socket.clone();
        // clone socket to socket_arc in order to share socket between threads
        println!("socket_arc: {:?}", socket_arc);

        let sender_arc = self.sender.clone();
        // clone sender to sender_arc in order to share sender between threads
        println!("sender_arc: {:?}", sender_arc);

        return thread::spawn(move || loop {
            // spawn thread to listen for requests and send them to sender thread to send requests and receive acknowledgements from sender thread

            let socket = socket_arc.lock().unwrap();
            // lock socket_arc to get socket to receive requests and send acknowledgements to sender thread on

            let sender = sender_arc.lock().unwrap();
            // lock sender_arc to get sender to send requests to sender thread

            let mut buf = [0; 100];
            // create buffer to hold message from sender thread

            let (_, src_addr) = (*socket).recv_from(&mut buf).expect("Didn't receive data");
            // receive message from sender thread and store message in buf and store sender thread address in src_addr

            // let buf_str = str::from_utf8(&buf[..]).unwrap();
            // println!("{}", buf_str);
            // println!("Got a message from {}", src_addr);

            if buf[0] == 2
            // asking for load
            {
                let reply_addr = src_addr;
                // clone src_addr to reply_addr to send reply to sender thread

                let message = load_arc.lock().unwrap();
                // lock load_arc to get load to hold number of requests received

                // println!("Load: {}",*message);

                let message = (*message).to_be_bytes();
                // convert load to bytes to send to sender thread

                // println!("{:?}", message);

                (*socket)
                    .send_to(&message, reply_addr) // send load to sender thread
                    .expect("Failed to send acknowledgement");
                // send load to src_addr to sender thread
            } else if buf[0] == 1 {
            }
            // do nothing
            else {
                // got a request from the client in buf

                let mut load = load_arc.lock().unwrap();
                // lock load_arc

                *load += 1;
                // increment load by 1 to keep track of number of requests received from sender thread and store in load variable

                drop(load);
                // drop load variable to unlock load_arc for other threads to use load_arc

                sender
                    .send((buf, src_addr))
                    // send request and src_addr to sender thread
                    .expect("Failed to pass request in queue");
                // add request to queue to be handled by sender thread
            }
            // end else
        });
        // end spawn
    }
    // end listen

    pub fn log_stats(&self) -> JoinHandle<()> {
        // log_stats function to log stats to file and return JoinHandle to join thread to main thread when finished logging stats to file

        let load_arc = self.load.clone();
        // clone load to load_arc in order to share load between threads

        let mut file = File::create({
            // create file to log stats to and store in file variable

            // create file to log stats
            let fname = self.addr.clone();
            // clone addr to fname to create file name

            format!("{}.txt", fname.replace(":", "-"))
            // return file name with .txt extension
        })
        // create file to log stats to and store in file variable
        .unwrap();
        // unwrap file creation to handle errors

        return thread::spawn(move || loop {
            // spawn thread to log stats to file and return JoinHandle to join thread to main thread when finished logging stats to file

            // spawn thread to log stats to file

            let load_val = {
                // get load value
                let load = load_arc.lock().unwrap();
                // lock load_arc

                *load
                // return load value
            };
            // end get load value

            if let Err(_) = writeln!(file, "{}", load_val)
            // write load value to file
            {
                println!("Failed to write to file");
            } // end if let

            thread::sleep(Duration::from_secs(1));
            // sleep for 1 second
            {
                () // do nothing
            } // end if let Err

            thread::sleep(Duration::from_secs(1));
            // sleep for 1 second
        });
        // end spawn
    }
    // end log_stats function

    // ring election functions

    pub fn handle_requests(&self) {
        // handle_requests function to handle requests from sender thread

        loop {
            // loop to handle requests from sender thread

            // read request from queue, send a reply, then sleep
            let (request, addr) = self.receiver.recv().unwrap();
            // get request from queue

            let mut load = self.load.lock().unwrap();
            // lock load

            *load -= 1;
            // decrement load

            drop(load);
            // dropping mutex early as we no longer need it

            println!("{}", String::from_utf8(request.to_vec()).unwrap());
            // print request to console

            // let reply = String::from("REQ PROCESSED");

            self.request_socket
                // send acknowledgement to sender thread
                .send_to(&request, addr)
                // send acknowledgement to sender thread
                .expect("Failed to send processed request");
            // send request to client as reply to request from client

            thread::sleep(Duration::from_micros(1000));
            // sleep for 1 millisecond
        }
        // end loop
    }
    // end handle_requests

    // function for election in a new thread
    pub fn election(&self) -> JoinHandle<()> {
        // election function to start election and return JoinHandle to join thread to main thread when finished election

        let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
        let port1 = "9000".parse::<u16>().unwrap();
        let port2 = "9001".parse::<u16>().unwrap();
        let port3 = "9002".parse::<u16>().unwrap();

        let addr1 = SocketAddr::new(ip, port1);
        let addr2 = SocketAddr::new(ip, port2);
        let addr3 = SocketAddr::new(ip, port3);

        let index_arc = self.index.clone();
        // clone index to index_arc in order to share index between threads

        let mut buf = [0; 1024];

        return thread::spawn(move || loop {
            if index_arc == 0 {
                let socket1 = UdpSocket::bind(addr1).unwrap();
                let index = "0";

                // let mut leader = "0";
                
                loop {
                    // let mut index_1 = "1";
                    // let mut index_2 = "2";

                    socket1.send_to(index.as_bytes(), addr2).unwrap();
                    socket1.send_to(index.as_bytes(), addr3).unwrap();

                    let (amt, src) = socket1.recv_from(&mut buf).unwrap();
                    println!("{}: {}", src, str::from_utf8(&buf[..amt]).unwrap());

                    // print buf to console
                    let buf_str = str::from_utf8(&buf[..amt]).unwrap().clone();
                    // clone buf_str to index_0 to store index of leader

                    if buf_str >= index {
                        // print
                        println!("input is greater than index");
                    }
                    else {
                        exit(0);
                    }

                    // print buf_str to console
                    // println!("buf_str: {}", buf_str);

                    // print src to console
                    // println!("src: {}", src);

                    // print amt to console
                    // println!("amt: {}", amt);

                    // index_0 = buf_str;

                    // if buf_str >= leader {
                    //     leader = buf_str;
                    // }

                    // if src is addr2 then index_1 = buf_str
                    // if src == addr2 {
                    //     let index_0 = str::from_utf8(&buf[..amt]).unwrap();
                    //     if index > index_0 {
                    //         break;
                    //     }
                    // }
                    // else if src == addr3 {
                    //     let index_1 = str::from_utf8(&buf[..amt]).unwrap();
                    //     if index > index_1 {
                    //         break;
                    //     }
                    // }

                    // print index_1 and index_2 to console
                    // println!("index_1: {}", index_1);
                    // println!("index_2: {}", index_2);

                    // index_0 = str::from_utf8(&buf[..amt]).unwrap();
                    // println!("index_0: {}", index_0);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            } 
            else if index_arc == 1 {
                // let mut index_0 = "0";
                // let mut index_2 = "2";

                let socket2 = UdpSocket::bind(addr2).unwrap();
                let index = "1";
                // let mut buf = [0; 1024];
                loop {
                    socket2.send_to(index.as_bytes(), addr1).unwrap();
                    socket2.send_to(index.as_bytes(), addr3).unwrap();

                    let (amt, src) = socket2.recv_from(&mut buf).unwrap();
                    println!("{}: {}", src, str::from_utf8(&buf[..amt]).unwrap());

                    // print buf to console
                    let buf_str = str::from_utf8(&buf[..amt]).unwrap();

                    if buf_str >= index {
                        // print
                        println!("input is greater than index");
                    }
                    else {
                        exit(0);
                    }

                    // print buf_str to console
                    // println!("buf_str: {}", buf_str);

                    // print src to console
                    // println!("src: {}", src);

                    // print amt to console
                    // println!("amt: {}", amt);

                    // if src == addr1 {
                    //     let index_0 = str::from_utf8(&buf[..amt]).unwrap();
                    //     if index > index_0 {
                    //         break;
                    //     }
                    // }
                    // else if src == addr3 {
                    //     let index_2 = str::from_utf8(&buf[..amt]).unwrap();
                    //     if index > index_2 {
                    //         break;
                    //     }
                    // }

                    // println!("index_1: {}", index_1);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            } 
            else if index_arc == 2 {
                // let index_0 = "0";
                // let index_1 = "1";

                let socket3 = UdpSocket::bind(addr3).unwrap();
                let index = "2";
                // let mut buf = [0; 1024];
                loop {
                    socket3.send_to(index.as_bytes(), addr1).unwrap();
                    socket3.send_to(index.as_bytes(), addr2).unwrap();
                    
                    let (amt, src) = socket3.recv_from(&mut buf).unwrap();
                    println!("{}: {}", src, str::from_utf8(&buf[..amt]).unwrap());

                    // print buf to console
                    let buf_str = str::from_utf8(&buf[..amt]).unwrap();

                    if buf_str >= index {
                        // print
                        println!("input is greater than index");
                    }
                    else {
                        exit(0);
                    }

                    // print buf_str to console
                    // println!("buf_str: {}", buf_str);

                    // print src to console
                    // println!("src: {}", src);
                    // print addr1 to console
                    // println!("addr1: {}", addr1);
                    // // print addr2 to console
                    // println!("addr2: {}", addr2);

                    // print amt to console
                    // println!("amt: {}", amt);

                    // if src == addr1 {
                    //     let index_0 = str::from_utf8(&buf[..amt]).unwrap();
                    //     if index > index_0 {
                    //         break;
                    //     }
                    // }
                    // else if src == addr2 {
                    //     let index_1 = str::from_utf8(&buf[..amt]).unwrap();
                    //     if index > index_1 {
                    //         break;
                    //     }
                    // }

                    // wait for 10 seconds
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
            break;
        });
    }
}

// end impl RequestReceiver to handle requests from sender thread
