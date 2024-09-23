mod socket;
mod commands;
mod tools;
mod rhythm;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {

    match rhythm::Rhythm::read_rhythm() {
        Ok(rhythm) => {
            socket::start_daemon(&rhythm).await
        },
        Err(e) => {
            eprintln!("Error: Failed to read rhythm for Bluebird: {}", e);
            Ok(())
        }
    }

}
