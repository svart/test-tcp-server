use std::cmp::{max, min};
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const ONE_HEADER_FIELD: usize = size_of::<u64>();
const HEADER_SIZE: usize = 2 * ONE_HEADER_FIELD;
const TCP_SEGMENT_SIZE: usize = 1442;
const SEND_BUFFER_SIZE: usize = 32 * TCP_SEGMENT_SIZE;

async fn server_connection_handler(mut sock: TcpStream) {
    let mut send_buf = [1u8; SEND_BUFFER_SIZE];
    loop {
        match sock.read_exact(&mut send_buf[..HEADER_SIZE]).await {
            Ok(0) | Err(_) => return,
            Ok(_) => {
                let _req_id = u64::from_be_bytes(send_buf[..ONE_HEADER_FIELD].try_into().expect("request should be ok"));
                let mut left_to_send = u64::from_be_bytes(send_buf[ONE_HEADER_FIELD..HEADER_SIZE].try_into().expect("size should be ok")) as usize;

                // println!("Received request {req_id} to send {left_to_send} bytes");

                // We need to send header at least
                left_to_send = max(left_to_send, HEADER_SIZE);

                while left_to_send > 0 {
                    let fun = if left_to_send < SEND_BUFFER_SIZE {
                        sock.write_all(&send_buf[..left_to_send])
                    } else {
                        sock.write_all(&send_buf)
                    };

                    match fun.await {
                        Ok(_) => {}
                        Err(_) => return,
                    }
                    left_to_send -= min(left_to_send, SEND_BUFFER_SIZE);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let local_address: SocketAddr = "0.0.0.0:12345".parse().unwrap();
    println!("Running TCP server listenning on {local_address}");

    let listen_sock = TcpListener::bind(local_address).await.expect("socket should bind normally");

    loop {
        match listen_sock.accept().await {
            Ok((socket, addr)) => {
                println!("Got connection from {addr}");
                tokio::spawn(server_connection_handler(socket));
            }
            Err(e) => println!("Accepting connection failed: {e}"),
        }
    }
}
