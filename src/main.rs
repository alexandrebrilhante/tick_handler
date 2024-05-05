use pulsar::{producer::ProducerOptions, Pulsar, TokioExecutor};
use std::error::Error;
use tokio::{io::AsyncReadExt, net::TcpListener, sync::mpsc};

/// Creates a Pulsar instance with the given address.
///
/// # Arguments
///
/// * `addr` - The address of the Pulsar instance.
///
/// # Returns
///
/// A `Result` containing the `Pulsar` instance or an error.
async fn create_pulsar(addr: &str) -> Result<Pulsar<TokioExecutor>, Box<dyn Error>> {
    Pulsar::builder(addr, TokioExecutor)
        .build()
        .await
        .map_err(|e: pulsar::Error| e.into())
}

/// Creates a Pulsar producer with the given Pulsar instance and topic name.
///
/// # Arguments
///
/// * `pulsar` - The Pulsar instance.
/// * `topic_name` - The name of the topic.
///
/// # Returns
///
/// A `Result` containing the `Producer` instance or an error.
async fn create_producer(
    pulsar: Pulsar<TokioExecutor>,
    topic_name: &str,
) -> Result<pulsar::Producer<TokioExecutor>, Box<dyn Error>> {
    pulsar
        .producer()
        .with_topic(topic_name)
        .with_name("producer")
        .with_options(ProducerOptions {
            batch_size: Some(4),
            ..Default::default()
        })
        .build()
        .await
        .map_err(|e: pulsar::Error| e.into())
}

/// Creates a channel for sending and receiving messages.
///
/// # Returns
///
/// A tuple containing the sender and receiver of the channel.
async fn create_channel() -> (mpsc::Sender<String>, mpsc::Receiver<String>) {
    mpsc::channel(100)
}

/// Creates a TCP listener with the given address.
///
/// # Arguments
///
/// * `addr` - The address to bind the listener to.
///
/// # Returns
///
/// A `Result` containing the `TcpListener` instance or an error.
async fn create_listener(addr: &str) -> Result<TcpListener, Box<dyn Error>> {
    TcpListener::bind(addr)
        .await
        .map_err(|e: std::io::Error| e.into())
}

/// Sends messages from the receiver to the Pulsar producer.
///
/// # Arguments
///
/// * `producer` - The Pulsar producer.
/// * `rx` - The receiver of the channel.
async fn send_messages(
    mut producer: pulsar::Producer<TokioExecutor>,
    mut rx: mpsc::Receiver<String>,
) {
    while let Some(message) = rx.recv().await {
        if let Err(e) = producer.send(message).await {
            eprintln!("Failed to send message to Pulsar; err = {:?}...", e);
        }
    }
}

/// Handles a connection from a TCP socket.
///
/// # Arguments
///
/// * `socket` - The TCP socket.
/// * `tx` - The sender of the channel.
async fn handle_connection(mut socket: tokio::net::TcpStream, tx: mpsc::Sender<String>) {
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => {
                let message: String = String::from_utf8_lossy(&buf[0..n]).to_string();

                if tx.send(message).await.is_err() {
                    eprintln!("Failed to send message to channel...");
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket; err = {:?}...", e);
                return;
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: &str = "pulsar://localhost:6650";
    let pulsar: Pulsar<TokioExecutor> = create_pulsar(addr).await?;

    let topic_name: &str = "persistent://public/default/test";
    let producer: pulsar::Producer<TokioExecutor> = create_producer(pulsar, topic_name).await?;

    let (tx, rx) = create_channel().await;

    let _producer_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        send_messages(producer, rx).await;
    });

    let listener: TcpListener = create_listener("127.0.0.1:9999").await?;

    loop {
        let (socket, _addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection...");

        let tx: mpsc::Sender<String> = tx.clone();

        tokio::spawn(async move {
            handle_connection(socket, tx).await;
        });
    }
}
