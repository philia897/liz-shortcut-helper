mod socket;
mod commands;
mod tools;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    socket::start_daemon().await
}
