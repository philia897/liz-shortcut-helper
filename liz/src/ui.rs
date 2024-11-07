use std::process::{Command, Stdio};
use std::io::Write;
use std::fmt;

#[derive(Debug)]
pub enum UiError {
    IoError(std::io::Error),
    OtherError(Box<dyn std::error::Error>),
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::IoError(err) => write!(f, "I/O error: {}", err),
            UiError::OtherError(err) => write!(f, "Other error: {}", err),
        }
    }
}

impl std::error::Error for UiError {}

impl From<std::io::Error> for UiError {
    fn from(err: std::io::Error) -> Self {
        UiError::IoError(err)
    }
}

impl From<Box<dyn std::error::Error>> for UiError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        UiError::OtherError(err)
    }
}

// Show the shortcuts using Rofi so that user can select
// Params : A vector of shortcut strings
// Return : The index of the selected item or None
pub fn show_shortcuts_rofi(options: &Vec<String>) -> Result<Option<String>, UiError> {
    // Join options into a single string for Rofi
    let options_str: String = options.join("\n");

    // Run Rofi to get user input
    // -i : case-insensitive
    // -p Liz: prompt
    // -markup-rows : enable html format
    // -format i : return the index of selected option (0-[N-1])
    let mut process = Command::new("rofi")
        .args(&["-dmenu", "-i", "-markup-rows", "-format", "i", "-p", "Liz"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write the options to Rofi's stdin
    if let Some(mut stdin) = process.stdin.take() {
        stdin
            .write_all(options_str.as_bytes())?;
    }

        // Wait for Rofi to finish and capture the selected option
    let output = process
        .wait_with_output()?;

    
    // Get the selected index from Rofi's output
    if output.status.success() {
        let selected_index: String = String::from_utf8_lossy(&output.stdout).trim()
            .to_string(); // Convert to String

        // If the output is empty, set action to "esc"
        if selected_index.is_empty() {
            return Ok(None);
        }

        // Return LizCommand with action "run" and args containing the selected index
        return Ok(Some(selected_index));
    } else {
        // If Rofi exits with an error, set action to "esc"
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Rofi exited with an error: {}", error_message);
        return Ok(None)
    }
}

// Define the ErrorType enum
#[derive(Debug)]
pub enum ErrorType {
    Running,
    // Unknown,
    Command,
    // Timeout,
}

// Define the show_error function
pub fn show_error(error_type: ErrorType, error_msg: &String) {
    match error_type {
        ErrorType::Running => {
            println!("Running error: {}", error_msg);
            Command::new("rofi")
                .args(&["-e", &format!("Running error: {}", error_msg)])
                .spawn()
                .expect("Failed to launch Rofi");
        }
        // ErrorType::Unknown => {
        //     println!("Unknown error: {}", error_msg);
        //     // Additional handling for unknown errors can go here
        // }
        ErrorType::Command => {
            println!("Command error: {}", error_msg);
            Command::new("rofi")
                .args(&["-e", &format!("Command error: {}", error_msg)])
                .spawn()
                .expect("Failed to launch Rofi");
        }
        // ErrorType::Timeout => {
        //     println!("Timeout error: {}", error_msg);
        //     // Additional handling for timeout errors can go here
        // }
    }
}

//TODO: complete the manual, maybe import from one file and show by rofi or commandline.
pub fn show_help() {
    let commands: Vec<(&str, &str, &str)> = vec![
        ("run", "", "Start liz and popup the shortcut helper."),
        ("reload", "DATA_PATH", "Reload shortcut data from sheets in the DATA_PATH (if given)."),
        ("persist", "", "Persist shortcut data manually."),
        ("info", "", "Show relative info of Liz."),
        ("help", "", "Show this help message."),
    ];

    println!("Usage: liz [COMMAND] [ARGUMENTS]");

    println!("\nAvailable Commands:");
    for (command, args, description) in commands {
        println!("  {:<10} {:<12} {}", command, args, description);
    }

    //println!("\nFor more information about a specific command, use `liz [COMMAND] --help`.");
}

pub fn show_info(items: &Vec<String>) {
    if items.len() % 2 != 0 {
        eprintln!("Warning: the length of results (item-content pairs) is {}, but it should be an even number.", items.len());
    }
    for chunk in items.chunks(2) {
        if let [key, value] = chunk {
            println!("{:<25}: {}", key, value);
        }
    }
}