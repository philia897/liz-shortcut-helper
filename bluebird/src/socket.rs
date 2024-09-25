use std::vec;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;
use tokio::time;
use crate::commands::Flute;
use crate::tools::db::DataTable;
use crate::tools::rhythm::Rhythm;
use andthe::{BlueBirdResponse, LizCommand, StateCode};

use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::flag::register;

pub async fn start_daemon() -> tokio::io::Result<()> {

    match Rhythm::read_rhythm() {
        Ok(rhythm) => {
            let flute_arc =  Arc::new(Mutex::new(
                Flute {
                    music_sheet : DataTable::import_from_json(&rhythm.music_sheet_path)
                                .expect(&format!("Failed to initialize the music_sheet from {}", rhythm.music_sheet_path)),
                    rhythm : rhythm
                }
            ));
            let wait_s: u64;
            let socket_path: String;
            {
                match flute_arc.lock() {
                    Ok(mut flute) => {
                        flute.calibrate();
                        wait_s = flute.rhythm.persist_freq_s;
                        socket_path = flute.rhythm.socket_path.clone();
                        let _ = std::fs::remove_file(socket_path.clone());
                    },
                    Err(e) => {
                        eprintln!("Failed to acquire the lock when initializing: {}", e);
                        return Ok(())
                    }
                }
            }
        
            let flute_arc_clone = flute_arc.clone();
            tokio::spawn(async move {
                persist_sheet(flute_arc_clone, wait_s).await;
            });
        
            tokio::spawn(async move {
                let _ = _start_daemon(flute_arc, socket_path).await;
            });
            
            // Set up signal handling with signal-hook
            let term_flag = Arc::new(AtomicBool::new(false));
            let term_flag_clone = Arc::clone(&term_flag);

            register(SIGINT, Arc::clone(&term_flag)).expect("Error setting signal handler");
            register(SIGTERM, term_flag).expect("Error setting signal handler");

            // Wait for the SIGINT/SIGTERM signal
            while !term_flag_clone.load(Ordering::Relaxed) {
                time::sleep(Duration::from_secs(1)).await;  // Check every second for the signal
            }

            println!("Received termination signal, shutting down gracefully...");

            Ok(())
        },
        Err(e) => {
            eprintln!("Error: Failed to read rhythm for Bluebird: {}", e);
            Ok(())
        }
    }
}

pub async fn _start_daemon(flute_arc: Arc<Mutex<Flute>>, socket_path: String) -> tokio::io::Result<()> {

    let listener: UnixListener = UnixListener::bind(socket_path).expect("Could not bind to socket");
    println!("A blue bird is listening...");

    let serious_error_response: Vec<u8> = BlueBirdResponse {
        code : StateCode::FAIL,
        results : vec!["BUG".to_string(), "A serious error occurred!".to_string(), "Please check the log of Bluebird.".to_string()]
    }.serialize().unwrap();
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buffer: Vec<u8> = vec![0u8; 1024];
        let n: usize = socket.read(&mut buffer).await?;
        if let Some(request) = LizCommand::deserialize(&buffer[..n]) {

            println!("Heard command: {:?}", request);

            let response: BlueBirdResponse;
            {
                match flute_arc.lock() {
                    Ok(mut flute) => {
                        response = flute.play(&request);
                    },
                    Err(e) => {
                        eprintln!("Failed to acquire the lock when initializing: {}", e);
                        continue;
                    }
                }
            }
    
            // Serialize command and send it to client
            if let Some(serialized) = response.serialize() {
                socket.write_all(&serialized).await?;
            } else {
                eprintln!("Failed to serialize the response: {:?}", response);
                socket.write_all(&serious_error_response).await?;
            }
        } else {
            eprintln!("Failed to deserialize the recieved Liz command (bytes): {:?}", &buffer[..n]);
            socket.write_all(&serious_error_response).await?;
        }
    }
}

// Function to save data to disk
async fn persist_sheet(flute_arc: Arc<Mutex<Flute>>, wait_s: u64) {
    loop {
        time::sleep(Duration::from_secs(wait_s)).await;
        match flute_arc.lock() {
            Ok(flute) => {
                // Persist the data automatically
                println!("Automatically persist the data into {}", flute.rhythm.music_sheet_path);
                let _ = flute.music_sheet.export_to_json(&flute.rhythm.music_sheet_path);
            },
            Err(e) => {
                eprintln!("Failed to acquire the lock when persisting sheet: {}", e);
            }
        }
    }
}