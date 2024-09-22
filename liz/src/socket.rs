use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};
use andthe::{BlueBirdResponse, LizCommand, StateCode};

use crate::ui::{show_error, show_shortcuts_rofi, ErrorType};

pub async fn send_command(command: &str, args: &Vec<String>) -> tokio::io::Result<()> {
    // Build the LizCommand
    let command: LizCommand = LizCommand {
        action: command.to_string(),
        args: args.to_vec(),
    };
    let _ = _send_command(command).await?;
    Ok(())
}

pub async fn _send_command(mut cmd: LizCommand) -> tokio::io::Result<()> {
    let socket_path: &str = "/tmp/rust_daemon.sock";
    
    let mut socket: UnixStream;

    let mut temp_buffer = vec![0u8; 1024]; // Temporary buffer for reading
    
    loop {
        socket = UnixStream::connect(socket_path).await?;

        // Serialize command and send it to daemon
        let serialized: Vec<u8> = serde_json::to_vec(&cmd).expect("Failed to serialize command");
        socket.write_all(&serialized).await?;
        
        // Receive the response from the daemon
        let mut buffer = Vec::new();  // Buffer to hold the entire respons

        // Read until there's no more data
        loop {
            let n = socket.read(&mut temp_buffer).await?;
            if n == 0 {
                break; // Connection closed
            }
            buffer.extend_from_slice(&temp_buffer[..n]);
        }

        // Deserialize the complete response
        let response: BlueBirdResponse = serde_json::from_slice(&buffer).expect("Failed to deserialize response");
        
        // Process the response
        let processed_response: Option<LizCommand> = process_response(&cmd, &response);
        match processed_response {
            None => {
                break;
            }
            Some(new_cmd) => {
                cmd = new_cmd;
            }
        }
    }

    Ok(())
}

// Function to process the response from the backend
fn process_response(cmd: &LizCommand, response: &BlueBirdResponse) -> Option<LizCommand> {

    // Check if the code is TIMEOUT or FAIL, and raise an error if so
    match response.code {
        StateCode::BUG => {
            eprintln!("Error: An unexpected bug occurred: {}", response.results.join("\n"));
            return None
        }
        StateCode::FAIL => {
            eprintln!("Error: Operation failed: {}", response.results.join("\n"));
            return None
        }
        _ => {} // pass
    }

    if response.results.is_empty() {
        println!("Ok");
        return None
    }

    match cmd.action.as_str() {
        "run" => {
            return process_response_of_run(response)
        },
        "reload" => {
            if response.code != StateCode::OK {
                show_error(ErrorType::Running, &response.results.join(" "));
            }
            return None
        }
        _ => {
            return None
        }
    }

}

fn process_response_of_run(response: &BlueBirdResponse) -> Option<LizCommand> {
    match show_shortcuts_rofi(&response.results) {
        Ok(Some(selected_index)) => {
            // Use the selected index (which is a String)
            println!("Selected index: {}", selected_index);
            let rst: LizCommand = LizCommand {
                action: "_exec_by_idx".to_string(),
                args: vec![selected_index]
            };

            return Some(rst);
        }
        Ok(None) => {
            // Handle the case where no selection was made
            println!("No selection made.");
            return None
        }
        Err(e) => {
            // Handle the error
            show_error(ErrorType::Running, &format!("{:?}", e));
            return None
        }
    }
}