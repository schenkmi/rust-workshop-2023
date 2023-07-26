//std::net::UdpSocket;

use std::str;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

const DISCOVERY_REQUEST: &str = "M-SEARCH * HTTP/1.1\r\n\
                                 HOST: 239.255.255.250:1900\r\n\
                                 MAN: \"ssdp:discover\"\r\n\
                                 MX: 2\r\n\
                                 ST: ssdp:all\r\n\
                                 \r\n";

fn main() -> std::io::Result<()> {
    {
        let any: SocketAddr = ([0, 0, 0, 0], 0).into();
        let socket = UdpSocket::bind(any)?;
        socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 255, 250), &Ipv4Addr::new(0, 0, 0, 0))?;

        // Set the socket address to the multicast IP and port for UPnP device discovery
        let socket_addr: SocketAddr = ([239, 255, 255, 250], 1900).into();

        // Send the discovery request
        socket.send_to(DISCOVERY_REQUEST.as_bytes(), &socket_addr)?;
        println!("Hello, world!");

       let mut buf = [0; 2048];
       //let (size, _) = socket.recv_from(&mut buf)?;
       match socket.recv(&mut buf) {
        Ok(received) => println!("received {received} bytes {:?}", &buf[..received]),
        Err(e) => println!("recv function failed: {e:?}"),
        }


       //let response =
       //str::from_utf8(unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) })?;

       //println!("{}", size);

        // // Receives a single datagram message on the socket. If `buf` is too small to hold
        // // the message, it will be cut off.
        // let mut buf = [0; 10];
        // let (amt, src) = socket.recv_from(&mut buf)?;

        // // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        // let buf = &mut buf[..amt];
        // buf.reverse();
        // socket.send_to(buf, &src)?;
    } // the socket is closed here
    Ok(())
}
