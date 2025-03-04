use crate::logger::LOGGER;
use crate::motor::*;
use crate::tools::RELAY;
use anyhow::{Ok, Result};
use libm;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

enum TestStatus {
    // TestFailed,   // 测试失败
    // TestSuccess,  // 测试成功
    Rotating,     // 转动中
    RotatFailed,  // 启动失败
    RotatSuccess, // 达到目标转速
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StartUpTestParam {
    target_rps: f32,
    total_count: u32,
    rotate_duration: u32,
    cold_duration: u32,
    has_relay: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TestResult {
    progress: Option<f32>,
    failed_cnt: Option<u32>,
    success_cnt: Option<u32>,
}

pub struct StartupTestHandle {
    running: Arc<AtomicBool>,
    handle: Mutex<Option<thread::JoinHandle<()>>>,
    total_cnt: Arc<Mutex<u32>>,
    failed_cnt: Arc<Mutex<u32>>,
    success_cnt: Arc<Mutex<u32>>,
}

pub static STARTUPTEST: Lazy<Mutex<StartupTestHandle>> =
    Lazy::new(|| Mutex::new(StartupTestHandle::new()));

impl StartupTestHandle {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: Mutex::new(None),
            total_cnt: Arc::new(Mutex::new(0)),
            failed_cnt: Arc::new(Mutex::new(0)),
            success_cnt: Arc::new(Mutex::new(0)),
        }
    }

    pub fn start(&self, test_param: StartUpTestParam) {
        let running = Arc::clone(&self.running);
        let mut handle_guard = self.handle.lock().unwrap();
        let total_cnt = Arc::clone(&self.total_cnt);
        let failed_cnt = Arc::clone(&self.failed_cnt);
        let success_cnt = Arc::clone(&self.success_cnt);

        // 重置计数
        *total_cnt.lock().unwrap() = test_param.total_count;
        *failed_cnt.lock().unwrap() = 0;
        *success_cnt.lock().unwrap() = 0;

        // logger::log_message(app, message, level);
        // LOGGER.lock().unwrap().log_message("开始测试", "info");

        if handle_guard.is_none() {
            running.store(true, Ordering::SeqCst);

            let handle = thread::spawn(move || {
                let mut test_cnt = 0;
                let mut reached_cnt = 0; // 达到目标转速计数
                while running.load(Ordering::SeqCst) {
                    {
                        // 电机上电
                        if test_param.has_relay {
                            RELAY.lock().unwrap().turn_on().unwrap();
                        }

                        // 等待电机上电完成
                        std::thread::sleep(std::time::Duration::from_millis(2000));

                        // 启动电机
                        MOTOR
                            .lock()
                            .unwrap()
                            .update_motor_speed_rps((test_param.target_rps * 100000.0) as u32)
                            .unwrap();
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        MOTOR.lock().unwrap().start_motor().unwrap();

                        test_cnt += 1;
                        let start = Instant::now();

                        LOGGER
                            .lock()
                            .unwrap()
                            .info(format!("开始第{}次测试", test_cnt).as_str());

                        loop {
                            let status = Self::get_test_status(&test_param.target_rps).unwrap();
                            match status {
                                TestStatus::Rotating => {}
                                TestStatus::RotatSuccess => {
                                    reached_cnt += 1;
                                } // 启动成功
                                _ => {
                                    reached_cnt = 0;
                                    *failed_cnt.lock().unwrap() += 1;
                                    LOGGER
                                        .lock()
                                        .unwrap()
                                        .danger(format!("第{}次测试未通过", test_cnt).as_str());
                                    break;
                                } // 启动失败
                            }

                            // 需要连续n秒钟都达到目标转速才判定为测试通过
                            if reached_cnt >= test_param.rotate_duration {
                                reached_cnt = 0;
                                *success_cnt.lock().unwrap() += 1;
                                LOGGER
                                    .lock()
                                    .unwrap()
                                    .info(format!("第{}次测试通过", test_cnt).as_str());

                                break;
                            }

                            if !running.load(Ordering::SeqCst) {
                                break;
                            }

                            std::thread::sleep(std::time::Duration::from_millis(1000));

                            // 启动超时则判定为启动失败
                            if start.elapsed().as_millis() > (test_param.rotate_duration * 1000 * 2) as u128 {
                                reached_cnt = 0;
                                *failed_cnt.lock().unwrap() += 1;
                                LOGGER
                                    .lock()
                                    .unwrap()
                                    .danger(format!("第{}次测试超时", test_cnt).as_str());
                                break;
                            }
                        }

                        MOTOR.lock().unwrap().stop_motor().unwrap();

                        // 电机断电
                        if test_param.has_relay {
                            RELAY.lock().unwrap().turn_off().unwrap();
                        }

                        // 等待电机停止转动并冷却
                        let start = Instant::now();
                        while running.load(Ordering::SeqCst) {
                            thread::sleep(std::time::Duration::from_millis(100));
                            if start.elapsed().as_millis() > (1000 * test_param.cold_duration) as u128 {
                                break;
                            }
                        }

                        if test_cnt >= test_param.total_count {
                            LOGGER.lock().unwrap().warning(format!("测试完成").as_str());
                            break;
                        }
                    }
                }
            });

            *handle_guard = Some(handle);
        }
    }

    pub fn stop(&self) {
        LOGGER.lock().unwrap().warning(format!("停止测试").as_str());

        let running = Arc::clone(&self.running);
        let mut handle_guard = self.handle.lock().unwrap();
        running.store(false, Ordering::SeqCst);

        if let Some(handle) = handle_guard.take() {
            handle.join().unwrap();
        }
    }

    fn get_test_status(rps: &f32) -> Result<TestStatus> {
        // 1. 检测状态, 若报错则停止电机、返回错误
        match MOTOR.lock().unwrap().get_motor_status() {
            std::result::Result::Ok(status) => {
                if status.error_code.unwrap() > 0 {
                    LOGGER
                        .lock()
                        .unwrap()
                        .warning(format!("故障状态: 0x{:x}", status.error_code.unwrap()).as_str());
                    return Ok(TestStatus::RotatFailed);
                }
            }
            Err(_) => {
                return Ok(TestStatus::RotatFailed);
            }
        }

        // 2. 检测转速, 达到目标转速且状态正常则判断为启动成功
        let curr_rps = MOTOR.lock().unwrap().get_current_rps().unwrap();
        if libm::fabs((curr_rps - rps) as f64) <= 0.5 {
            return Ok(TestStatus::RotatSuccess);
        }

        Ok(TestStatus::Rotating)
    }

    pub fn get_test_result(&self) -> Result<TestResult> {
        let total_cnt = Arc::clone(&self.total_cnt);
        let failed_cnt = Arc::clone(&self.failed_cnt);
        let success_cnt = Arc::clone(&self.success_cnt);

        let f_cnt = *failed_cnt.lock().unwrap();
        let s_cnt = *success_cnt.lock().unwrap();
        let mut t_cnt = *total_cnt.lock().unwrap();

        if t_cnt == 0 {
            t_cnt = 1;
        }

        Ok(TestResult {
            // progress: Some(*test_cnt as f32 / *total_cnt as f32 + 0.001),
            progress: Some(((f_cnt + s_cnt) / t_cnt) as f32),
            failed_cnt: Some(f_cnt),
            success_cnt: Some(s_cnt),
        })
    }
}
