use crate::motor::*;
use anyhow::Result;
use serialport;
use std::time::Duration;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn init_serial_port(sp: &str, baud: u32) -> CmdResult {
    // 初始化 SerialPort 实例
    let result = serialport::new(sp, baud)
        .timeout(Duration::from_millis(100))
        .open();

    match result {
        Ok(port_new) => {
            MOTOR.lock().unwrap().port = Some(port_new);
            MOTOR.lock().unwrap().reset_motor().unwrap();
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
pub async fn get_motor_params() -> CmdResult<MotorParams> {
    let params = MOTOR.lock().unwrap().get_motor_params().unwrap();

    Ok(params)
}

#[tauri::command]
pub async fn get_motor_status() -> CmdResult<MotorStatus> {
    let params = MOTOR.lock().unwrap().get_motor_status().unwrap();

    Ok(params)
}

#[tauri::command]
pub async fn update_motor_rps(rps: f32, poles: u32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_speed_hz((rps * poles as f32 * 1000.0) as u32)
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
pub async fn start_motor(rps: f32, poles: u32) -> CmdResult {
    MOTOR
        .lock()
        .unwrap()
        .update_motor_speed_hz((rps * poles as f32 * 1000.0) as u32)
        .unwrap();
    MOTOR.lock().unwrap().start_motor().unwrap();

    Ok(())
}

#[tauri::command]
pub async fn stop_motor() -> CmdResult {
    MOTOR.lock().unwrap().update_motor_speed_hz(0).unwrap();
    MOTOR.lock().unwrap().stop_motor().unwrap();
    MOTOR.lock().unwrap().reset_motor().unwrap();
    Ok(())
}
