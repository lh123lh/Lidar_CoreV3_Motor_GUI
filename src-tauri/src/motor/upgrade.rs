use crate::motor::*;
use crate::ymodem::xymodem_util;
use crate::ymodem::ymodem::Ymodem;
use anyhow::{bail, Ok, Result};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Upgrade {
    pub progress: Arc<Mutex<f32>>,
}

pub static UPGRADE: Lazy<Mutex<Upgrade>> = Lazy::new(|| Mutex::new(Upgrade::new()));

impl Upgrade {
    pub fn new() -> Self {
        Upgrade {
            progress: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn start(&self, sp: &str, baud: u32, fw_path: &str) -> Result<()> {
        // 打开文件
        let file = File::open(fw_path).unwrap();
        let file_len = file.metadata().unwrap().len();
        let path = std::path::Path::new(fw_path);
        let filename = path.file_name().unwrap();
        // 创建一个BufReader来读取文件
        let mut reader = BufReader::new(file);

        // 释放MOTOR串口
        MOTOR.lock().unwrap().port = None;
        // 重新打开串口
        let mut port = serialport::new(sp, baud)
            .timeout(Duration::from_millis(200))
            .open()?;

        // 进入IAP模式
        let cmd: Vec<u8> = vec![0x75, 0x70, 0x64, 0x61, 0x74, 0x65]; // update
        let mut connect_cnt = 0;

        loop {
            port.write(&cmd).unwrap();
            match (xymodem_util::get_stream_timeout(&mut port))? {
                Some(c) => {
                    if c[3] == crate::ymodem::ymodem::CRC {
                        dbg!("Enter IAP Mode Success");
                        break;
                    }
                }
                None => {}
            }

            connect_cnt += 1;

            if connect_cnt >= 200 {
                eprint!("Enter IAP Mode Timeout",);
                break;
            }
        }

        let mut ymodem = Ymodem::new();

        match ymodem.send(
            &mut port,
            &mut reader,
            filename.to_str().unwrap().to_string(),
            file_len,
        ) {
            core::result::Result::Ok(_) => {}
            Err(err) => {
                bail!("{:?}", err)
            }
        }

        Ok(())
    }
}
