use anyhow::{bail, Ok, Result};
use once_cell::sync::Lazy;
use serialport::{self, SerialPort};
use std::sync::Mutex;
use std::{thread, time::Duration};

pub struct Relay {
    pub port: Option<Box<dyn SerialPort>>,
}

pub static RELAY: Lazy<Mutex<Relay>> = Lazy::new(|| Mutex::new(Relay::new()));

impl Relay {
    pub fn new() -> Self {
        Relay { port: None }
    }

    pub fn turn_on(&mut self) -> Result<()> {
        let mut cmds: Vec<[u8; 8]> = Vec::new();
        cmds.push([0x01, 0x05, 0x00, 0x00, 0xFF, 0x00, 0x8C, 0x3A]);
        cmds.push([0x01, 0x05, 0x00, 0x01, 0xFF, 0x00, 0xDD, 0xFA]);
        cmds.push([0x01, 0x05, 0x00, 0x02, 0xFF, 0x00, 0x2D, 0xFA]);
        cmds.push([0x01, 0x05, 0x00, 0x03, 0xFF, 0x00, 0x7C, 0x3A]);

        if let Some(ref mut port) = self.port {
            for cmd in cmds {
                thread::sleep(Duration::from_millis(30));
                match port.write(&cmd) {
                    core::result::Result::Ok(_) => {
                        // return Ok(());
                    }
                    Err(e) => {
                        bail!(e);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<()> {
        let mut cmds: Vec<[u8; 8]> = Vec::new();
        cmds.push([0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0xCD, 0xCA]);
        cmds.push([0x01, 0x05, 0x00, 0x01, 0x00, 0x00, 0x9C, 0x0A]);
        cmds.push([0x01, 0x05, 0x00, 0x02, 0x00, 0x00, 0x6C, 0x0A]);
        cmds.push([0x01, 0x05, 0x00, 0x03, 0x00, 0x00, 0x3D, 0xCA]);

        if let Some(ref mut port) = self.port {
            for cmd in cmds {
                thread::sleep(Duration::from_millis(30));
                match port.write(&cmd) {
                    core::result::Result::Ok(_) => {
                        // return Ok(());
                    }
                    Err(e) => {
                        bail!(e);
                    }
                }
            }
        }

        Ok(())
    }
}
