// use  std::sync::mpsc::Receiver;
use std::net::UdpSocket; // use UdpSocket module from net module to create socket and address for receiver and sender
use std::io; // use io module to allow for reading from stdin 

pub struct RequestSender{ // struct to hold sender info to send requests and receive acknowledgements
    addr: String, // local address to send requests from
    socket: UdpSocket, // socket to send requests and receive acknowledgements
    recepient_addr: String, // address to send requests
}

impl RequestSender{ // impl RequestSender to allow for creating RequestSender objects 
    pub fn new(addr: String, recepient_addr: String) -> RequestSender { // constructor of RequestSender
        RequestSender{ // return RequestSender struct
            addr: addr.clone(), // set addr 
            recepient_addr: recepient_addr.clone(), // set recepient_addr 
            socket: UdpSocket::bind(addr).expect("couldn't bind reciever to address") // bind socket to addr 
        } // return RequestSender 
    } // end constructor 

    pub fn run(&self){ // run function to send requests and receive acknowledgements
        loop { // loop to allow for sending multiple requests 
            let mut message = String::new(); // create string to hold message 
            io::stdin().read_line(&mut message).expect("failed to read line"); // read message from stdin 
            io::stdin().read_line(&mut message).unwrap(); // read message from stdin 
            let mut message = String::from(message.trim()).into_bytes(); // convert message to bytes 
            message.insert(0,0); // insert 0 at beginning of message to indicate that it is a request 
            self.socket.send_to(&message, self.recepient_addr.clone()).unwrap(); // send message to recepient_addr 
        } // end loop
    } // end run function
}