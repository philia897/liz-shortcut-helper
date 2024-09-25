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
    pub ydotook_interval_ms: u64, // for ydotool config
}

impl Default for Rhythm {
    fn default() -> Self {
        // Get the home directory and construct the rhythm path
        let home: String = home_dir().unwrap_or_else(|| "~/".into()).to_string_lossy().into_owned();
        let liz_path: String = format!("{}/.config/liz", home);
        let user_sheets_path: String = format!("{}/sheets", liz_path);
        let socket_path: String = "/tmp/bluebird_daemon.sock".to_string();
        let music_sheet_path: String = format!("{}/music_sheet.lock", liz_path);
        let keymap_path: String = format!("{}/keymap.json", liz_path);
        let ydotool_socket_path = "/tmp/.ydotool_socket".to_string();

        Self {
            liz_path: liz_path,
            user_sheets_path: user_sheets_path,
            socket_path: socket_path,
            music_sheet_path: music_sheet_path,
            keymap_path: keymap_path,
            persist_freq_s: 3600,
            ydotool_socket_path: ydotool_socket_path,
            ydotook_interval_ms: 100
        }
    }
}

impl Rhythm {
    pub fn read_rhythm() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rhythm_path: PathBuf = home_dir().ok_or("Failed to retrieve home directory")?;
        rhythm_path.push(".config/liz/rhythm.toml");
    
        let content: String = fs::read_to_string(rhythm_path)?;
        let rhythm: Rhythm = toml::de::from_str(&content).unwrap_or_default();
    
        eprintln!("Reading Config:\n{:?}", rhythm);

        Ok(rhythm)
    }
}