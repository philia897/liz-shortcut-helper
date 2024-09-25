use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn get_args(shortcut: &str) -> Vec<&str> {
    if shortcut.starts_with("+") {
        let type_str: &str = &shortcut[2..]; // remove the prefix
        vec!["type", type_str]
    } else {
        let mut cmd: Vec<&str> = vec!["key"];
        let keys: Vec<&str> = shortcut.split_whitespace().collect(); // Collect keys as Vec<&str>
        cmd.extend(keys);
        cmd
    }
}

pub fn execute_shortcut_ydotool(shortcut_str: &str, delay_ms: u64, ydotool_socket_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let shortcuts: Vec<&str> = shortcut_str.split("<str>").collect();

    for shortcut in shortcuts {
        if shortcut.is_empty() {
            continue;
        }

        sleep(Duration::from_millis(delay_ms)); // Sleep for the specified delay

        let args: Vec<&str> = get_args(shortcut);

        // Execute the command
        let result: Result<std::process::Output, std::io::Error> = Command::new("ydotool")
            .env("YDOTOOL_SOCKET", ydotool_socket_path)
            .args(args)
            .output();

        match result {
            Ok(output) => {        // Log the output and error
                if output.status.success() {
                    // println!("Command executed successfully: {:?}", output);
                } else {
                    // Explicitly return an Err with the error message
                    let error_message: String = format!(
                        "ydotool command failed with status: {:?}, stderr: {:?}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_message)))
                }
            }
            Err(e) => {
                return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Failed to execute command: {}", e))))
            }
        }
    }

    Ok(())
}