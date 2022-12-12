use tracing::instrument;
use tracing::{info, info_span};
use tracing_subscriber::prelude::*;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::{error::Error, io, net::SocketAddr};

mod custom_layer;
use custom_layer::CustomLayer;
#[path = "fmt/yak_shave.rs"]
mod yak_shave;

#[tracing::instrument]
async fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
    let stream = TcpStream::connect(&addr).await;
    tracing::info!("created stream");
    stream
}

#[tracing::instrument]
async fn write(stream: &mut TcpStream) -> io::Result<usize> {
    let result = stream.write(b"hello world\n").await;
    info!("wrote to stream; success={:?}", result.is_ok());
    result
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let addr = "127.0.0.1:6142".parse()?;
    tracing_subscriber::registry().with(CustomLayer).init();
    
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = connect(&addr).await?;

    write(&mut stream).await?;


    //Demo for a sync function


    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    tracing::info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    tracing::info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
    Ok(())

    
}