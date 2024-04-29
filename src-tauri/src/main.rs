// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod cmds;
mod motor;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::init_serial_port,
            cmds::deinit_serial_port,
            cmds::get_motor_params,
            cmds::get_motor_status,
            cmds::list_avaliable_ports,
            cmds::update_motor_rps,
            cmds::enable_motor_identify,
            cmds::enable_motor_rs_online,
            cmds::enable_motor_rs_recalc,
            cmds::clear_motor_faults,
            cmds::update_motor_acc_max,
            cmds::update_motor_acc_start,
            cmds::start_motor,
            cmds::stop_motor,
            cmds::upload_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
