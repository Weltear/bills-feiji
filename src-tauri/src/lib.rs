pub mod time;
pub mod bill;
pub mod datainterface;
pub mod account;
pub mod statistics;
pub mod config;

use datainterface::{greet, get_appointed_data, save_appointed_data, get_hello_data,
    big_acc_data_load, big_acc_data_save, get_config, get_stat_data, save_config, get_day_stat,
    get_pie_data};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_appointed_data,
            save_appointed_data,
            get_hello_data,
            big_acc_data_load,
            big_acc_data_save,
            get_config,
            get_stat_data,
            save_config,
            get_day_stat,
            get_pie_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
