use anyhow::{bail, Context, Ok, Result};
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
}

pub struct Motor {
    pub port: Option<Box<dyn SerialPort>>,
}

pub static MOTOR: Lazy<Mutex<Motor>> = Lazy::new(|| Mutex::new(Motor::new()));

impl Motor {
    pub fn new() -> Self {
        Motor { port: None }
    }

    #[allow(unused_assignments)]
    pub fn get_motor_params(&mut self) -> Result<MotorParams> {
        let mut rs = 0.0;
        let mut rs_online = 0.0;
        let mut ls_d = 0.0;
        let mut ls_q = 0.0;
        let mut flux = 0.0;

        rs = self.request(0x1, 0).unwrap() as f64 / 100000000.0;
        rs_online = self.request(0x2, 0).unwrap() as f64 / 100000000.0;
        ls_d = self.request(0x3, 0).unwrap() as f64 / 100000000.0;
        ls_q = self.request(0x4, 0).unwrap() as f64 / 100000000.0;
        flux = self.request(0x5, 0).unwrap() as f64 / 100000000.0;

        Ok(MotorParams {
            rs: Some(rs),
            rs_online: Some(rs_online),
            ls_d: Some(ls_d),
            ls_q: Some(ls_q),
            flux: Some(flux),
        })
    }

    fn request(&mut self, msg_type: u8, msg: u32) -> Option<u32> {
        let mut cmd: Vec<u8> = vec![];
        cmd.push(0x5a);
        cmd.push(0x5a);
        cmd.push(msg_type);
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
                    let mut buf: Vec<u8> = vec![0; 16];

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

                            let value: u32 = ((buf[3] as i32) << 24
                                | (buf[4] as i32) << 16
                                | (buf[5] as i32) << 8
                                | (buf[6] as i32))
                                as u32;
                            return Some(value);
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
}
