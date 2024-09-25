use andthe::{BlueBirdResponse, LizCommand, StateCode};
use serde::{Deserialize, Serialize};

use crate::tools::{db::{DataTable, UserDataTable}, exec::execute_shortcut_ydotool, rhythm::Rhythm};

#[derive(Debug, Serialize, Deserialize)]
pub struct Flute {
    pub music_sheet : DataTable,
    pub rhythm : Rhythm
}

impl Flute {

    pub fn calibrate(&mut self) -> &mut Self {
        self.update_rank();
        self
    }

    fn update_rank(&mut self) {
        self.music_sheet.sort_by_column("formatted", true);
        self.music_sheet.sort_by_column("hit_number", false);
    }

    pub fn play(&mut self, cmd: &LizCommand) -> BlueBirdResponse {
        match cmd.action.as_str() {
            "run" => self.run(cmd),
            "reload" => self.reload(cmd),
            "_exec_by_idx" => self._exec_by_idx(cmd),
            "persist" => self.persist(cmd),
            _ => self.default_handle(cmd),
        }
    }
    
    fn run(&self, _cmd: &LizCommand) -> BlueBirdResponse {
        BlueBirdResponse {
            code : StateCode::OK,
            results : self.music_sheet.get_formatted_vec()
        }
    }
    
    fn reload(&mut self, cmd: &LizCommand) -> BlueBirdResponse {
        let user_data_path: &String;
        if cmd.args.is_empty() {
            // eprintln!("BUG: Empty args, expect one path string on args[0]");
            // return BlueBirdResponse {
            //     code : StateCode::FAIL,
            //     results : vec!["Failure:".to_string(), "Empty args:".to_string(), "Expect one path string".to_string()]
            // }
            user_data_path = &self.rhythm.user_sheets_path;
        } else {
            user_data_path = &cmd.args[0];
        }
        match UserDataTable::import_from(&user_data_path) {
            Ok(user_data) => {
                self.music_sheet = user_data.transform_to_data_table(&self.music_sheet,&self.rhythm.keymap_path).expect("222");
            }
            Err(e) => {
                eprintln!("Failure: failed to import user data from: {}, error: {}", user_data_path, e);
                return BlueBirdResponse {
                    code : StateCode::FAIL,
                    results : vec!["Failure:".to_string(), "Failed to import:".to_string(), user_data_path.to_string()]
                }
            }
        }
        BlueBirdResponse {
            code : StateCode::OK,
            results : vec!["Reload Done".to_string()]
        }
    }
    
    fn _exec_by_idx(&mut self, cmd: &LizCommand) -> BlueBirdResponse {
        if cmd.args.is_empty() {
            eprintln!("BUG: Empty args, expect one index on args[0]");
            return BlueBirdResponse {
                code : StateCode::BUG,
                results : vec!["BUG:".to_string(), "Empty args:".to_string(), "Expect one index".to_string()]
            }
        }
        match cmd.args[0].parse::<usize>() {
            Ok(idx) => {
                let keycode = self.music_sheet.get_value(idx, "keycode");
                if keycode.is_none() {
                    eprintln!("BUG: No keycode found on index {}", cmd.args[0]);
                    return BlueBirdResponse {
                        code : StateCode::BUG,
                        results : vec!["BUG:".to_string(), "No keycode found on index:".to_string(), cmd.args[0].clone()]
                    }
                }
                if let Err(e) = execute_shortcut_ydotool(
                            &keycode.unwrap(), self.rhythm.ydotook_interval_ms, &self.rhythm.ydotool_socket_path) {
                    eprintln!("Failure: Fail to execute shortcut: {:?}", e);
                    return BlueBirdResponse {
                        code : StateCode::FAIL,
                        results : vec!["Failure:".to_string(), format!("{}", e)]
                    }
                }
                let _ = self.music_sheet.hit_num_up(idx);
                self.update_rank();
                return BlueBirdResponse {
                    code : StateCode::OK,
                    results : vec![]
                }
            },
            Err(_e) => {
                eprintln!("BUG: Parsing this index error: {}", cmd.args[0]);
                return BlueBirdResponse {
                    code : StateCode::BUG,
                    results : vec!["BUG:".to_string(), "Parsing this index error:".to_string(), cmd.args[0].clone()]
                }
            },
        }
    }
    
    fn persist(&self, _cmd: &LizCommand) -> BlueBirdResponse {
        match self.music_sheet.export_to_json(&self.rhythm.music_sheet_path) {
            Ok(()) => {
                BlueBirdResponse{
                    code : StateCode::OK,
                    results : vec![]
                }
            },
            Err(e) => {
                eprintln!("BUG: Failed to persist music_sheet, error: {}", e);
                BlueBirdResponse{
                    code : StateCode::BUG,
                    results : vec!["BUG:".to_string(), "Failed to persist music_sheet".to_string()]
                }
            }
        }
    }

    fn default_handle(&self, cmd: &LizCommand) -> BlueBirdResponse {
        BlueBirdResponse {
            code : StateCode::FAIL,
            results : vec![cmd.action.to_string(), "Invalid".to_string()]
        }
    }
}

// Implement the Drop trait for Flute
impl Drop for Flute {
    fn drop(&mut self) {
        // Attempt to save the music_sheet when the Flute instance is dropped
        let file_path: &String = &self.rhythm.music_sheet_path;
        if let Err(e) = self.music_sheet.export_to_json(file_path) {
            eprintln!("Failed to save music sheet in Drop: {}", e);
        } else {
            println!("Music sheet saved successfully in Drop.");
        }
    }
}
