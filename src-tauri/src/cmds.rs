use crate::{motor::*, tools};
use anyhow::Result;
use serde_json::Value;
use serialport;
use std::time::Duration;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn init_serial_port(sp: &str, baud: u32) -> CmdResult {
    // 初始化 SerialPort 实例
    let result = serialport::new(sp, baud)
        .timeout(Duration::from_millis(50))
        .open();

    match result {
        Ok(port_new) => {
            MOTOR.lock().unwrap().port = Some(port_new);
            // MOTOR.lock().unwrap().reset_motor().unwrap();
            return Ok(());
        }
        Err(_) => {
            return Err("Failed To connect port".to_string());
        }
    }
}

#[tauri::command]
pub async fn deinit_serial_port() -> CmdResult {
    MOTOR.lock().unwrap().port = None;
    Ok(())
}

#[tauri::command]
pub async fn list_avaliable_ports() -> CmdResult<Vec<String>> {
    let mut ports = Vec::new();
    let ports_info = serialport::available_ports().expect("No ports found!");
    for p in ports_info {
        ports.push(p.port_name);
    }

    Ok(ports)
}

#[tauri::command]
pub async fn get_motor_current_rps() -> CmdResult<f32> {
    let rps = MOTOR.lock().unwrap().get_current_rps().unwrap();

    Ok(rps)
}

#[tauri::command]
pub async fn get_motor_current_pos() -> CmdResult<f32> {
    let pos = MOTOR.lock().unwrap().get_current_pos().unwrap();

    Ok(pos)
}

#[tauri::command]
pub async fn get_motor_params() -> CmdResult<MotorParams> {
    let params = MOTOR.lock().unwrap().get_motor_params().unwrap();

    Ok(params)
}

#[tauri::command]
pub async fn get_motor_static_params() -> CmdResult<MotorStaticParams> {
    let params = MOTOR.lock().unwrap().get_motor_static_params().unwrap();

    Ok(params)
}

#[tauri::command]
pub async fn get_motor_status() -> CmdResult<MotorStatus> {
    let status = MOTOR.lock().unwrap().get_motor_status().unwrap();

    Ok(status)
}

#[tauri::command]
pub async fn get_motor_special_params() -> CmdResult<MotorSpecialParams> {
    let params = MOTOR.lock().unwrap().get_motor_special_params().unwrap();

    Ok(params)
}

#[tauri::command]
pub async fn update_motor_rps(rps: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_speed_rps((rps * 100000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn enable_motor_identify(en: bool) -> CmdResult {
    MOTOR.lock().unwrap().set_motor_identify_enable(en).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn enable_motor_rs_online(en: bool) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .set_motor_rs_online_enable(en)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn enable_motor_rs_recalc(en: bool) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .set_motor_rs_recalc_enable(en)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn clear_motor_faults() -> CmdResult {
    MOTOR.lock().unwrap().clear_motor_faults().unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_acc_max(hz: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_acc_max((hz * 1000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_acc_start(hz: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_acc_start((hz * 1000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_kp_spd(kp: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_kp_spd((kp * 100000000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_ki_spd(ki: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_ki_spd((ki * 100000000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_kp_iq(kp: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_kp_iq((kp * 100000000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_ki_iq(ki: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_ki_iq((ki * 100000000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn start_motor(rps: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_speed_rps((rps * 100000.0) as u32)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    MOTOR.lock().unwrap().start_motor().unwrap();

    Ok(())
}

#[tauri::command]
pub async fn stop_motor() -> CmdResult {
    MOTOR.lock().unwrap().update_motor_speed_hz(0).unwrap();
    MOTOR.lock().unwrap().stop_motor().unwrap();
    // MOTOR.lock().unwrap().reset_motor().unwrap();
    Ok(())
}

#[tauri::command]
pub async fn enable_motor_pos_ctrl(en: bool, mode: u8) -> CmdResult {
    MOTOR.lock().unwrap().set_motor_pos_ctrl_enable(en, mode).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_position(pos: f32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_position((pos * 1000.0) as u32)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn update_motor_special_params(param: MotorSpecialParams) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_special_params(param)
        .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn export_motor_special_params(param: MotorSpecialParams, path: String) -> CmdResult {
    // 检查路径是否合法
    if path.is_empty() {
        return Err("path is invalid".to_string());
    }

    MOTOR
        .lock()
        .unwrap()
        .export_motor_special_params(param, path)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn import_motor_special_params(path: String) -> CmdResult<Value> {
    // 检查路径是否合法
    if path.is_empty() {
        return Err("path is invalid".to_string());
    }

    let params = MOTOR
        .lock()
        .unwrap()
        .import_motor_special_params(path)
        .unwrap();

    Ok(params)
}

#[tauri::command]
pub async fn upload_file(file: std::path::PathBuf) -> Result<String, String> {
    // 在这里处理文件，例如保存到磁盘或进一步处理
    println!("{:?}", file);
    Ok("File uploaded successfully".to_string())
}

#[tauri::command]
pub async fn start_record_rps(path: String) -> CmdResult {
    // 检查路径是否合法
    if path.is_empty() {
        return Err("path is invalid".to_string());
    }

    MOTOR.lock().unwrap().start_rps_record(&path).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn stop_record_rps() -> CmdResult {
    MOTOR.lock().unwrap().stop_rps_record().unwrap();

    Ok(())
}

#[tauri::command]
pub async fn start_startup_task(rps: f32, count: u32, cold: u32) -> CmdResult {
    STARTUPTEST.lock().unwrap().start(rps, count, cold);

    Ok(())
}

#[tauri::command]
pub async fn stop_startup_task() -> CmdResult {
    STARTUPTEST.lock().unwrap().stop();
    Ok(())
}

#[tauri::command]
pub async fn get_startup_test_result() -> CmdResult<TestResult> {
    let result = STARTUPTEST.lock().unwrap().get_test_result().unwrap();

    Ok(result)
}

#[tauri::command]
pub async fn upgrade_motor_fw(path: String, sp: String, baud: u32) -> CmdResult {
    // 检查路径是否合法
    if path.is_empty() {
        return Err("path is invalid".to_string());
    }

    match UPGRADE.lock().unwrap().start(&sp, baud, &path) {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

/// 合并电机驱动固件, 合成固件的原理其实很简单, 就是将BootLoader的Hex文件的最后两行删除, 将App的Hex文件第一行删除, 然后将Application的Hex文件所有内容复制到BootLoader的Hex文件的最后面即可
#[tauri::command]
pub async fn merge_firmware(boot: String, app: String, output: String) -> CmdResult {
    tools::merge_file(&boot, &app, &output, 0, -2, 1, 0).unwrap();

    Ok(())
}
