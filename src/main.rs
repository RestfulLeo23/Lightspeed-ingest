mod ftl_codec;
use bytes::{Buf, BufMut, BytesMut};
use ftl_codec::*;
use futures::{stream, SinkExt, StreamExt};
use hex::encode;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio_util::codec::{Decoder, Encoder, Framed};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Listening on port 8084");
    let mut listener = TcpListener::bind("0.0.0.0:8084").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    println!("Sender addr: {:?}", stream.peer_addr().unwrap());
    let mut frame = Framed::new(stream, FtlCodec::new());
    loop {
        match frame.next().await {
            Some(result) => match result {
                Ok(command) => {
                    println!("Command was {:?}", command);
                    handle_command(command, &mut frame);
                    return;
                }
                Err(e) => {
                    println!("There was an error: {:?}", e);
                    return;
                }
            },
            None => {
                println!("There was a socket reading error");
                return;
            }
        };
    }
}

async fn handle_command(command: FtlCommand, frame: &mut Framed<TcpStream, FtlCodec>) {
    match command.command {
        Command::HMAC => {
            println!("Handling HMAC Command");
            let hmac = generate_hmac();
            println!("payload generated {:?}", hmac);
            let stream = ["200 ", hmac.as_str(), "/n"];
            match frame
                .send_all(&mut stream::iter(stream.iter()).map(|x| Ok(x)))
                .await
            {
                Ok(_) => {
                    println!("hmac sent");
                    return;
                }
                Err(e) => {
                    println!("There was an error {:?}", e);
                    return;
                }
            }
        }
        _ => {
            println!("Command not implemented yet. Tell GRVY to quit his day job");
            return;
        }
    }
}

fn generate_hmac() -> String {
    let dist = Uniform::new(0x00, 0xFF);
    let mut hmac_payload: Vec<u8> = Vec::new();
    let mut rng = thread_rng();
    for i in 0..128 {
        hmac_payload.push(rng.sample(dist));
    }
    encode(hmac_payload.as_slice())
}
