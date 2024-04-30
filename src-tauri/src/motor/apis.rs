use anyhow::{Ok, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serialport::{self, SerialPort};
use std::sync::Mutex;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorParams {
    pub rs: Option<f64>,
    pub rs_online: Option<f64>,
    pub ls_d: Option<f64>,
    pub ls_q: Option<f64>,
    pub flux: Option<f64>,
    pub poles: Option<u32>,
    pub target_rps: Option<f32>,
    pub torque: Option<f64>,
    pub vdc_bus: Option<f64>,
    pub acc_max_hzps: Option<f64>,
    pub acc_start_hzps: Option<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorVariableParams {
    pub acc_max_hzps: Option<f64>,
    pub acc_start_hzps: Option<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorStatus {
    pub rps: Option<f32>,
    pub identified: Option<bool>,
    pub error_code: Option<u16>,
    pub motor_state: Option<String>,
    pub mctrl_state: Option<String>,
    pub identified_en: Option<bool>,
}

pub struct Motor {
    pub port: Option<Box<dyn SerialPort>>,
    pub recoder_handle: Option<Box<csv::Writer<std::fs::File>>>,
}

#[derive(Debug, Serialize)]
struct RpsRecoder {
    rps: Option<f32>,
}

pub static MOTOR: Lazy<Mutex<Motor>> = Lazy::new(|| Mutex::new(Motor::new()));

impl Motor {
    pub fn new() -> Self {
        Motor {
            port: None,
            recoder_handle: None,
        }
    }

    fn motor_state_to_string(&mut self, state: &u8) -> String {
        match state {
            0 => String::from("STOP_IDLE"),
            1 => String::from("BRAKE_STOP"),
            2 => String::from("SEEK_POS"),
            3 => String::from("ALIGNMENT"),
            4 => String::from("OL_START"),
            5 => String::from("CL_RUNNING"),
            _ => String::from("CTRL_RUN"),
        }
    }

    fn mctrl_state_to_string(&mut self, state: &u8) -> String {
        match state {
            0 => String::from("INIT_SET"),
            1 => String::from("FAULT_STOP"),
            2 => String::from("BRAKE_STOP"),
            3 => String::from("FIRST_RUN"),
            4 => String::from("NORM_STOP"),
            _ => String::from("CONT_RUN"),
        }
    }

    #[allow(unused_assignments)]
    pub fn get_motor_params(&mut self) -> Result<MotorParams> {
        let mut rs = 0.0;
        let mut rs_online = 0.0;
        let mut ls_d = 0.0;
        let mut ls_q = 0.0;
        let mut flux = 0.0;
        let mut poles = 0;
        let mut target_rps = 0.0;
        let mut torque = 0.0;
        let mut vdc_bus = 0.0;
        let mut acc_max_hzps = 0.0;
        let mut acc_start_hzps = 0.0;

        if let Some(buf) = self.request(0x7F, 0) {
            rs = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
            rs_online = vec_to_int(&buf[4..8]) as f64 / 100000000.0;
            ls_d = vec_to_int(&buf[8..12]) as f64 / 100000000.0;
            ls_q = vec_to_int(&buf[12..16]) as f64 / 100000000.0;
            flux = vec_to_int(&buf[16..20]) as f64 / 100000000.0;
            poles = vec_to_int(&buf[20..24]) as u32;
            target_rps = vec_to_int(&buf[24..28]) as f32 / 1000.0;
            torque = vec_to_int(&buf[28..32]) as f64 / 1000.0;
            vdc_bus = vec_to_int(&buf[32..36]) as f64 / 1000.0;
            acc_max_hzps = vec_to_int(&buf[36..40]) as f64 / 1000.0;
            acc_start_hzps = vec_to_int(&buf[40..44]) as f64 / 1000.0;
        }

        // 记录转速
        if let Some(ref mut wtr) = self.recoder_handle {
            wtr.serialize(RpsRecoder {
                rps: Some(target_rps),
            })?;

            wtr.flush()?;
        }

        Ok(MotorParams {
            rs: Some(rs),
            rs_online: Some(rs_online),
            ls_d: Some(ls_d),
            ls_q: Some(ls_q),
            flux: Some(flux),
            poles: Some(poles),
            target_rps: Some(target_rps),
            torque: Some(torque),
            vdc_bus: Some(vdc_bus),
            acc_max_hzps: Some(acc_max_hzps),
            acc_start_hzps: Some(acc_start_hzps),
        })
    }

    #[allow(unused_assignments)]
    pub fn get_motor_status(&mut self) -> Result<MotorStatus> {
        let mut rps: f32 = 0.0;
        let mut identified = false;
        let mut error_code = 0;
        let mut motor_state = String::from("IDEL");
        let mut mctrl_state = String::from("IDEL");
        let mut identified_en = false;

        if let Some(buf) = self.request(0x06, 0) {
            rps = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            identified = buf[4] != 0;
            error_code = vec_to_short(&buf[5..7]) as u16;
            motor_state = self.motor_state_to_string(&buf[7]);
            mctrl_state = self.mctrl_state_to_string(&buf[8]);
            identified_en = &buf[9].into() != 0;
        }

        Ok(MotorStatus {
            rps: Some(rps),
            identified: Some(identified),
            error_code: Some(error_code),
            motor_state: Some(motor_state),
            mctrl_state: Some(mctrl_state),
            identified_en: Some(identified_en),
        })
    }

    pub fn update_motor_speed_hz(&mut self, speed_hz: u32) -> Result<()> {
        if let Some(_) = self.request(0x81, speed_hz) {}
        Ok(())
    }

    pub fn start_motor(&mut self) -> Result<()> {
        if let Some(_) = self.request(0x82, 0) {}
        Ok(())
    }

    pub fn stop_motor(&mut self) -> Result<()> {
        if let Some(_) = self.request(0x83, 0) {}
        Ok(())
    }

    pub fn reset_motor(&mut self) -> Result<()> {
        if let Some(_) = self.request(0x84, 0) {}
        Ok(())
    }

    pub fn set_motor_identify_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(0x86, en as u32) {}
        Ok(())
    }

    pub fn set_motor_rs_online_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(0x87, en as u32) {}
        Ok(())
    }

    pub fn set_motor_rs_recalc_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(0x88, en as u32) {}
        Ok(())
    }

    pub fn clear_motor_faults(&mut self) -> Result<()> {
        if let Some(_) = self.request(0x89, 0) {}
        Ok(())
    }

    pub fn update_motor_acc_max(&mut self, hz: u32) -> Result<()> {
        if let Some(_) = self.request(0x8A, hz) {}
        Ok(())
    }

    pub fn update_motor_acc_start(&mut self, hz: u32) -> Result<()> {
        if let Some(_) = self.request(0x8B, hz) {}
        Ok(())
    }

    fn request(&mut self, msg_type: u8, msg: u32) -> Option<Vec<u8>> {
        let mut cmd: Vec<u8> = vec![];
        cmd.push(0x5a);
        cmd.push(0x5a);
        cmd.push(msg_type);
        cmd.push(4);
        cmd.push(((msg >> 24) & 0xff) as u8);
        cmd.push(((msg >> 16) & 0xff) as u8);
        cmd.push(((msg >> 8) & 0xff) as u8);
        cmd.push(((msg >> 0) & 0xff) as u8);
        cmd.push(0xa5);
        cmd.push(0xa5);

        if let Some(ref mut port) = self.port {
            match port.write(&cmd) {
                core::result::Result::Ok(_) => {
                    // read msg
                    let mut buf: Vec<u8> = vec![0; 128];

                    match port.read(buf.as_mut_slice()) {
                        core::result::Result::Ok(t) => {
                            // TODO: 增强数据帧校验
                            if t < 9
                                || buf[0] != 0x5a
                                || buf[1] != 0x5a
                                || buf[t - 1] != 0xa5
                                || buf[t - 2] != 0xa5
                            {
                                return None;
                            }

                            let len = buf[3] as usize;
                            return Some(buf[4..len + 4].to_vec());
                        }
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return None;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    return None;
                }
            }
        }

        None
    }

    pub fn start_rps_record(&mut self, file_name: &str) -> Result<()> {
        let wtr = csv::Writer::from_path(file_name)?;
        self.recoder_handle = Some(Box::new(wtr));
        Ok(())
    }

    pub fn stop_rps_record(&mut self) -> Result<()> {
        self.recoder_handle = None;
        Ok(())
    }
}

pub fn vec_to_int(buf: &[u8]) -> u32 {
    ((buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32)) as u32
}

pub fn vec_to_short(buf: &[u8]) -> i16 {
    ((buf[0] as i16) << 8 | (buf[1] as i16)) as i16
}
