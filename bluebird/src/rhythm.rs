use std::fs;
use serde::{Deserialize, Serialize};
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct  Rhythm {
    pub rhythm_path : String,
    pub user_sheets_path : String,
    pub socket_path : String,
    pub music_sheet_path : String,
    pub keymap_path : String
}

impl Rhythm {
    pub fn read_rhythm() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rhythm_path = home_dir().ok_or("Failed to retrieve home directory")?;
        rhythm_path.push(".config/liz/rhythm.toml");
    
        let content: String = fs::read_to_string(rhythm_path)?;
        let rhythm: Rhythm = toml::from_str(&content)?;
    
        Ok(rhythm)
    }
}