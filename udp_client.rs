use std::net::UdpSocket;
use std::io;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:4445")?;

    loop {
        let mut input = String::new();
        println!("Enter message: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "bye" {
            break;
        }

        socket.send(input.as_bytes())?;

        let mut buffer = [0u8; 1024];
        socket.recv(&mut buffer)?;
        println!("Server response: {}", str::from_utf8(&buffer).expect("Could not read buffer as string"));
    }

    Ok(())
}

