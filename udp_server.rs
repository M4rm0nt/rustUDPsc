use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:4445")?;
    println!("Server running on 127.0.0.1:4445");

    loop {
        let mut buffer = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut buffer)?;

        let received = str::from_utf8(&buffer[..amt]).expect("Could not write buffer as string");
        println!("Received from {}: {}", src, received);

        let response = format!("Received your message: {}", received);
        socket.send_to(response.as_bytes(), &src)?;
    }
}

