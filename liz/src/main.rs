use ui::{show_error, show_help};

mod ui;
mod socket;

struct UserCommand {
    command : String,
    args : Vec<String>
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    
    // If command is valid, send it to the daemon
    if let Some(cmd) = get_user_command() {

        // TODO: considering add functionality that allow more command and arguments

        socket::send_command(&cmd.command, &cmd.args).await
    } else {
        Ok(()) // directly return
    }
}

fn get_user_command() -> Option<UserCommand> {
    // Get the command-line arguments
    let parsed_args: Vec<String> = std::env::args().collect();

    let command : String;
    let args : Vec<String>;
    // Check if there's a valid command after the program name
    if parsed_args.len() > 2 {
        command = parsed_args[1].to_string();
        args = parsed_args[2..].to_vec();
    } else if parsed_args.len() > 1 {
        command = parsed_args[1].to_string();
        args = vec![];
    } else {
        command = "run".to_string(); // Default command
        args = vec![];
    }

    match command.as_str() {
        "run" | "reload" | "persist" => {},  // Valid command
        "help" => {
            show_help();
            return None
        }
        _ => {
            show_error(ui::ErrorType::Command, &format!("Invalid command '{}', Use 'liz help'", command));
            return None  // Return None for an invalid command
        }
    }

    let cmd: UserCommand = UserCommand {
        command : command,
        args : args
    };

    Some(cmd)
}