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
        Ok(port) => {
            // 将其存储在状态中
            MOTOR.lock().unwrap().port = Some(port);
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
pub async fn get_motor_params() -> CmdResult<MotorParams> {
    let params = MOTOR.lock().unwrap().get_motor_params().unwrap();

    Ok(params)
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
