// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use logger::LOGGER;

mod cmds;
mod logger;
mod motor;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            LOGGER.lock().unwrap().app_handle = Some(Box::new(app_handle));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmds::init_serial_port,
            cmds::deinit_serial_port,
            cmds::get_motor_current_rps,
            cmds::get_motor_params,
            cmds::get_motor_status,
            cmds::get_motor_static_params,
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
            cmds::start_record_rps,
            cmds::stop_record_rps,
            cmds::start_startup_task,
            cmds::stop_startup_task,
            cmds::get_startup_test_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
