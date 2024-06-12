// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use logger::LOGGER;

mod cmds;
mod logger;
mod motor;
mod tools;
mod ymodem;

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
            cmds::get_motor_current_pos,
            cmds::get_motor_params,
            cmds::get_motor_status,
            cmds::get_motor_static_params,
            cmds::get_motor_special_params,
            cmds::list_avaliable_ports,
            cmds::update_motor_rps,
            cmds::enable_motor_identify,
            cmds::enable_motor_rs_online,
            cmds::enable_motor_rs_recalc,
            cmds::clear_motor_faults,
            cmds::update_motor_acc_max,
            cmds::update_motor_acc_start,
            cmds::update_motor_kp_spd,
            cmds::update_motor_ki_spd,
            cmds::update_motor_kp_iq,
            cmds::update_motor_ki_iq,
            cmds::start_motor,
            cmds::stop_motor,
            cmds::enable_motor_pos_ctrl,
            cmds::update_motor_position,
            cmds::update_motor_special_params,
            cmds::export_motor_special_params,
            cmds::import_motor_special_params,
            cmds::upload_file,
            cmds::start_record_rps,
            cmds::stop_record_rps,
            cmds::start_startup_task,
            cmds::stop_startup_task,
            cmds::get_startup_test_result,
            cmds::upgrade_motor_fw,
            cmds::merge_firmware,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
