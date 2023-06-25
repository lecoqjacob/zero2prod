use tokio::{io, net::TcpListener};
use zero2prod::run;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    run(listener).await
}