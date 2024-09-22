use std::{process, vec};
use std::thread;
use std::sync::{Arc, Mutex};

use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::commands::Flute;
use crate::tools::db::DataTable;
use andthe::{BlueBirdResponse, LizCommand, StateCode};

pub struct  Rhythm {
    pub socket_path : String,
    pub music_sheet_path : String,
    pub keymap_path : String
}

pub async fn start_daemon(rhythm : &Rhythm) -> tokio::io::Result<()> {
    let socket_path: &String = &rhythm.socket_path;

    let mut flute: Flute = Flute {
        music_sheet : DataTable::import_from_json(&rhythm.music_sheet_path).expect("Failed to initialize the music_sheet"),
        keymap_file : rhythm.keymap_path.clone(),
        music_sheet_path : rhythm.music_sheet_path.clone()
    };
    flute.calibrate();

    // persist the music_sheet and clean up resources when exiting
    // data_maintain(&flute, rhythm);

    let _ = std::fs::remove_file(socket_path);

    let listener: UnixListener = UnixListener::bind(socket_path).expect("Could not bind to socket");
    println!("Daemon started, waiting for requests...");

    let serious_error_response: Vec<u8> = BlueBirdResponse {
        code : StateCode::FAIL,
        results : vec!["BUG".to_string(), "A serious error occurred!".to_string(), "Please check the log of Bluebird.".to_string()]
    }.serialize().unwrap();
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buffer: Vec<u8> = vec![0u8; 1024];
        let n: usize = socket.read(&mut buffer).await?;
        if let Some(request) = LizCommand::deserialize(&buffer[..n]) {

            let response: BlueBirdResponse  = flute.play(&request);
    
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

fn data_maintain(flute: Arc<Mutex<Flute>>, rhythm: Arc<Mutex<Rhythm>>) {
    // Create a new Signals instance to handle signals
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).expect("Unable to create Signals instance");

    // Spawn a thread to handle signals
    thread::spawn(move || {
        for _ in signals.forever() {
            let flute = flute.lock().unwrap();
            let rhythm = rhythm.lock().unwrap();
            flute.music_sheet.export_to_json(&rhythm.music_sheet_path);
            println!("Exiting...");
            process::exit(0);
        }
    });
}