use socket::Rhythm;

mod socket;
mod commands;
mod tools;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // let rst: Option<String> = tools::db::convert_shortcut_to_key_presses("meta+pageup tab 123!@# tab ABC", "./data/keymap.json");
    // println!("{}", rst.unwrap_or_default());
    // let rst = tools::exec::execute_shortcut_ydotool("126.1 104.1 104.0 126.0", 100).expect("Execute shortcut failed");
    // let user_data: tools::db::UserDataTable = tools::db::UserDataTable::import_from_json("input.json").expect("111");
    // let old_mng = tools::db::DataTable::import_from_json("input-lock.json").expect("222");
    // let mut manager = user_data.transform_to_data_table(&old_mng, "./data/keymap.json").expect("222");
    // let idx = 1;
    // manager.sort_by_column("formatted", true);
    // manager.sort_by_column("hit_number", false);
    // let _ = manager.hit_num_up(idx);
    // let aaa = manager.get_formatted_vec();
    // println!("{:?}", aaa);
    // println!("{}", manager.get_value(idx, "keycode").expect("cc"));
    // let _ = manager.export_to_json("input-lock.json");

    let rhythm: Rhythm = Rhythm {
        socket_path : "/tmp/rust_daemon.sock".to_string(),
        music_sheet_path : "input-lock.json".to_string(),
        keymap_path : "./data/keymap.json".to_string()
    };

    socket::start_daemon(&rhythm).await
}
