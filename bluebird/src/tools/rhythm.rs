use std::fs;
use serde::{Deserialize, Serialize};
use dirs::home_dir;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct  Rhythm {
    pub liz_path : String,   // The config path from 
    pub user_sheets_path : String,  // Path for all the shortcut sheets
    pub socket_path : String,  // Path for socket for communication between Liz and Bluebird
    pub music_sheet_path : String,  // Path for the lock file for Bluebird
    pub keymap_path : String,
    pub persist_freq_s : u64,  // The interval between two auto-persisting
    pub ydotool_socket_path : String,  // for ydotool config
    pub interval_ms: u64, // for ydotool config
}

impl Default for Rhythm {
    fn default() -> Self {
        // Get the home directory and construct the rhythm path
        let home: String = home_dir().unwrap_or_else(|| "~/".into()).to_string_lossy().into_owned();
        let liz_path: String = format!("{}/.config/liz", home);
        let user_sheets_path: String = format!("{}/sheets", liz_path);
        // let socket_path: String = format!("{}/tmp/bluebird_daemon.sock", liz_path);
        let socket_path: String = "/tmp/bluebird_daemon.sock".to_string();
        let music_sheet_path: String = format!("{}/music_sheet.lock", liz_path);
        let keymap_path: String = format!("{}/keymap_builtin.json", liz_path);

        // By default bluebird uses Enigo for key events.
        // If this is given, it will use ydotool for key events instead.
        let ydotool_socket_path = "".to_string();

        Self {
            liz_path: liz_path,
            user_sheets_path: user_sheets_path,
            socket_path: socket_path,
            music_sheet_path: music_sheet_path,
            keymap_path: keymap_path,
            persist_freq_s: 3600,
            ydotool_socket_path: ydotool_socket_path,
            interval_ms: 100
        }
    }
}

impl Rhythm {
    pub fn read_rhythm() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rhythm_path: PathBuf = home_dir().ok_or("Failed to retrieve home directory")?;
        rhythm_path.push(".config/liz/rhythm.toml");
    
        if !rhythm_path.exists() {
            eprintln!("Warning: rhythm config file not found, using default values.");
            return Ok(Rhythm::default());
        }

        let content: String = fs::read_to_string(rhythm_path)?;
        let rhythm: Rhythm = toml::de::from_str(&content).unwrap_or_default();

        Ok(rhythm)
    }

    pub fn to_pretty_vec(&self) -> Vec<String> {
        vec![
            "liz_path".to_string(), self.liz_path.clone(),
            "user_sheets_path".to_string(), self.user_sheets_path.clone(),
            "socket_path".to_string(), self.socket_path.clone(),
            "music_sheet_path".to_string(), self.music_sheet_path.clone(),
            "keymap_path".to_string(), self.keymap_path.clone(),
            "persist_freq_s".to_string(), self.persist_freq_s.to_string(),
            "ydotool_socket_path".to_string(), self.ydotool_socket_path.clone(),
            "interval_ms".to_string(), self.interval_ms.to_string(),
        ]
    }
}