use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {

        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }

        // write every single byte in the buffer
        writer.write_all(line.as_bytes()).await.unwrap();
    }
}
