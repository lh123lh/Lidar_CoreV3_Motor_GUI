use crate::tools;
use anyhow::{bail, Ok, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serialport::{self, SerialPort};
use std::sync::Mutex;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorParams {
    pub vdc_bus: Option<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorStaticParams {
    pub rs: Option<f64>,
    pub ls_d: Option<f64>,
    pub ls_q: Option<f64>,
    pub flux: Option<f64>,
    pub poles: Option<u32>,
    pub kp_spd: Option<f64>,
    pub ki_spd: Option<f64>,
    pub kp_iq: Option<f64>,
    pub ki_iq: Option<f64>,
    pub main_version: Option<u8>,
    pub sub_version: Option<u8>,
    pub rev_version: Option<u8>,
    pub stage_version: Option<char>,
    pub version_date: Option<u32>,
    pub acc_max_hzps: Option<f64>,
    pub acc_start_hzps: Option<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorStatus {
    pub identified: Option<bool>,
    pub error_code: Option<u16>,
    pub motor_state: Option<String>,
    pub mctrl_state: Option<String>,
    pub rsonline_en: Option<bool>,
    pub rsrecalc_en: Option<bool>,
}

#[derive(Debug, Serialize)]
struct RpsRecoder {
    rps: Option<f32>,
}

#[allow(dead_code)]
enum GetCmdTypes {
    GetMotorRps,
    GetMotorPos,
    GetVersion,
    GetVersionDate,
    GetParamRs,
    GetParamRsOnline,
    GetParamLsD,
    GetParamLsQ,
    GetParamFlux,
    GetMotorStatus,
    GetMctrlStatus,
    GetPolePairs,
    GetTorque,
    GetVdcBus,
    GetIa0,
    GetIa1,
    GetIa2,
    GetVv0,
    GetVv1,
    GetVv2,
    GetAccStart,
    GetAccMax,
    GetPidSpd,
    GetPidIQ,
    GetResEstCurrent,
    GetIndEstCurrent,
    GetMaxCurrent,
    GetFluxExecFreq,
    GetWbpKgm2,
    GetRatedVoltage,
    GetFluxCurrent,
    GetAlignCurrent,
    GetStartupCurrent,
    GetTorqueCurrent,
    GetSpeedStart,
    GetSpeedForce,
    GetOverCurrentFault,
    GetOverVoltageFault,
    GetUnderVoltageFault,
    GetOverLoadPower,
    GetStallCurrent,
    GetFaultCheckCurrent,
    GetFailSpeedMax,
    GetFailSpeedMin,
    GetEncSlots,
    GetAdcScaleVol,
    GetAdcScaleCur,
    GetAdcVolFilterPole,
    GetAdcOffACurrent,
    GetAdcOffBCurrent,
    GetAdcOffCCurrent,
    GetAdcOffAVoltage,
    GetAdcOffBVoltage,
    GetAdcOffCVoltage,
}

#[allow(dead_code)]
enum SetCmdTypes {
    SetMotorSpeedRps = 0x80,
    SetMotorSpeedHz,
    SetMotorStart,
    SetMotorStop,
    SetMotorReset,
    SetEnableIndentify,
    SetEnableRsOnline,
    SetEnableRsRecalc,
    SetClearFaults,
    SetAccMax,
    SetAccStart,
    SetKpSpd,
    SetKiSpd,
    SetKpIq,
    SetKiIq,
    SetEnablePosCtrl,
    SetMotorPosition,
    SetParamRs,
    SetParamLsD,
    SetParamLsQ,
    SetParamFlux,
    SetPolePairs,
    SetResEstCurrent,
    SetIndEstCurrent,
    SetMaxCurrent,
    SetFluxExecFreq,
    SetWbpKgm2,
    SetRatedVoltage,
    SetFluxCurrent,
    SetAlignCurrent,
    SetStartupCurrent,
    SetTorqueCurrent,
    SetSpeedStart,
    SetSpeedForce,
    SetOverCurrentFault,
    SetOverVoltageFault,
    SetUnderVoltageFault,
    SetOverLoadPower,
    SetStallCurrent,
    SetFaultCheckCurrent,
    SetFailSpeedMax,
    SetFailSpeedMin,
    SetEncSlots,
    SetAdcScaleVol,
    SetAdcScaleCur,
    SetAdcVolFilterPole,
    SetAdcOffACurrent,
    SetAdcOffBCurrent,
    SetAdcOffCCurrent,
    SetAdcOffAVoltage,
    SetAdcOffBVoltage,
    SetAdcOffCVoltage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorFeatureParams {
    pub poles: Option<u32>,
    pub rs_ohm: Option<f32>,
    pub ls_d: Option<f32>,
    pub ls_q: Option<f32>,
    pub rated_flux: Option<f32>,
    pub res_est_current: Option<f32>,
    pub ind_est_current: Option<f32>,
    pub max_current: Option<f32>,
    pub flux_exec_freq: Option<f32>,
    pub wbp_kgm2: Option<f32>,
    pub rated_voltage: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorStartupParams {
    pub flux_current: Option<f32>,
    pub align_current: Option<f32>,
    pub startup_current: Option<f32>,
    pub torque_current: Option<f32>,
    pub speed_start: Option<f32>,
    pub speed_force: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorFaultChkParams {
    pub over_current: Option<f32>,
    pub over_voltage: Option<f32>,
    pub under_voltage: Option<f32>,
    pub over_load_power: Option<f32>,
    pub stall_current: Option<f32>,
    pub fault_ckeck_current: Option<f32>,
    pub fail_speed_max: Option<f32>,
    pub fail_speed_min: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorEncoderParams {
    pub slots: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorAdcParams {
    pub vol_scale: Option<f32>,
    pub cur_scale: Option<f32>,
    pub vol_filter_pole: Option<f32>,
    pub off_a_cur: Option<f32>,
    pub off_b_cur: Option<f32>,
    pub off_c_cur: Option<f32>,
    pub off_a_vol: Option<f32>,
    pub off_b_vol: Option<f32>,
    pub off_c_vol: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MotorSpecialParams {
    pub feature_param: MotorFeatureParams,
    pub startup_param: MotorStartupParams,
    pub fault_check_param: MotorFaultChkParams,
    pub encoder_param: MotorEncoderParams,
    pub adc_param: MotorAdcParams,
}

pub struct Motor {
    pub port: Option<Box<dyn SerialPort>>,
    pub recoder_handle: Option<Box<csv::Writer<std::fs::File>>>,
}

pub static MOTOR: Lazy<Mutex<Motor>> = Lazy::new(|| Mutex::new(Motor::new()));

#[allow(dead_code)]
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

    pub fn get_current_rps(&mut self) -> Result<f32> {
        let mut rps: f32 = -1000.0;

        if let Some(buf) = self.request(GetCmdTypes::GetMotorRps as u8, 0) {
            if buf.len() >= 4 {
                rps = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        // 记录转速
        if let Some(ref mut wtr) = self.recoder_handle {
            wtr.serialize(RpsRecoder { rps: Some(rps) })?;

            wtr.flush()?;
        }

        Ok(rps)
    }

    pub fn get_current_pos(&mut self) -> Result<f32> {
        let mut pos: f32 = -1000.0;

        if let Some(buf) = self.request(GetCmdTypes::GetMotorPos as u8, 0) {
            if buf.len() >= 4 {
                pos = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        Ok(pos)
    }

    pub fn get_motor_static_params(&mut self) -> Result<MotorStaticParams> {
        let mut rs = 0.0;
        let mut ls_d = 0.0;
        let mut ls_q = 0.0;
        let mut flux = 0.0;
        let mut poles = 0;
        let mut kp_spd = 0.0;
        let mut ki_spd = 0.0;
        let mut kp_iq = 0.0;
        let mut ki_iq = 0.0;
        let mut main_version = 0;
        let mut sub_version = 0;
        let mut rev_version = 0;
        let mut stage_version = 'a';
        let mut version_date = 0;
        let mut acc_max_hzps = 0.0;
        let mut acc_start_hzps = 0.0;

        if let Some(buf) = self.request(GetCmdTypes::GetParamRs as u8, 0) {
            if buf.len() >= 4 {
                rs = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetVersion as u8, 0) {
            if buf.len() >= 4 {
                main_version = buf[0];
                sub_version = buf[1];
                rev_version = buf[2];
                stage_version = buf[3] as char;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetVersionDate as u8, 0) {
            if buf.len() >= 4 {
                version_date = vec_to_int(&buf[0..4]) as u32;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetParamLsD as u8, 0) {
            if buf.len() >= 4 {
                ls_d = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetParamLsQ as u8, 0) {
            if buf.len() >= 4 {
                ls_q = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetParamFlux as u8, 0) {
            if buf.len() >= 4 {
                flux = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetPolePairs as u8, 0) {
            if buf.len() >= 4 {
                poles = vec_to_int(&buf[0..4]) as u32;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAccStart as u8, 0) {
            if buf.len() >= 4 {
                acc_start_hzps = vec_to_int(&buf[0..4]) as f64 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAccMax as u8, 0) {
            if buf.len() >= 4 {
                acc_max_hzps = vec_to_int(&buf[0..4]) as f64 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetPidSpd as u8, 0) {
            if buf.len() >= 8 {
                kp_spd = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
                ki_spd = vec_to_int(&buf[4..8]) as f64 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetPidIQ as u8, 0) {
            if buf.len() >= 8 {
                kp_iq = vec_to_int(&buf[0..4]) as f64 / 100000000.0;
                ki_iq = vec_to_int(&buf[4..8]) as f64 / 100000000.0;
            }
        }

        Ok(MotorStaticParams {
            rs: Some(rs),
            ls_d: Some(ls_d),
            ls_q: Some(ls_q),
            flux: Some(flux),
            poles: Some(poles),
            kp_spd: Some(kp_spd),
            ki_spd: Some(ki_spd),
            kp_iq: Some(kp_iq),
            ki_iq: Some(ki_iq),
            main_version: Some(main_version),
            sub_version: Some(sub_version),
            rev_version: Some(rev_version),
            stage_version: Some(stage_version),
            version_date: Some(version_date),
            acc_max_hzps: Some(acc_max_hzps),
            acc_start_hzps: Some(acc_start_hzps),
        })
    }

    #[allow(unused_assignments)]
    pub fn get_motor_params(&mut self) -> Result<MotorParams> {
        let mut vdc_bus = 0.0;

        if let Some(buf) = self.request(GetCmdTypes::GetVdcBus as u8, 0) {
            if buf.len() >= 4 {
                vdc_bus = vec_to_int(&buf[0..4]) as f64 / 1000.0;
            }
        }

        Ok(MotorParams {
            vdc_bus: Some(vdc_bus),
        })
    }

    #[allow(unused_assignments)]
    pub fn get_motor_status(&mut self) -> Result<MotorStatus> {
        let mut identified = false;
        let mut error_code = 0;
        let mut motor_state = String::from("IDEL");
        let mut mctrl_state = String::from("IDEL");
        let mut rsonline_en = false;
        let mut rsrecalc_en = false;

        if let Some(buf) = self.request(GetCmdTypes::GetMotorStatus as u8, 0) {
            if buf.len() >= 4 {
                error_code = vec_to_short(&buf[1..3]) as u16;
                motor_state = self.motor_state_to_string(&buf[3]);
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetMctrlStatus as u8, 0) {
            if buf.len() >= 4 {
                mctrl_state = self.mctrl_state_to_string(&buf[0]);
                identified = buf[1] != 0;
                rsonline_en = buf[2] != 0;
                rsrecalc_en = buf[3] != 0;
            }
        }

        Ok(MotorStatus {
            identified: Some(identified),
            error_code: Some(error_code),
            motor_state: Some(motor_state),
            mctrl_state: Some(mctrl_state),
            rsonline_en: Some(rsonline_en),
            rsrecalc_en: Some(rsrecalc_en),
        })
    }

    pub fn get_motor_special_params(&mut self) -> Result<MotorSpecialParams> {
        let motor_feature_param = self.get_motor_feature_params().unwrap();

        let motor_startup_param = self.get_motor_startup_params().unwrap();

        let motor_faultchk_param = self.get_motor_faultchk_params().unwrap();

        let motor_encoder_param = self.get_motor_encoder_params().unwrap();

        let motor_adc_param = self.get_motor_adc_params().unwrap();

        Ok(MotorSpecialParams {
            feature_param: motor_feature_param,
            startup_param: motor_startup_param,
            fault_check_param: motor_faultchk_param,
            encoder_param: motor_encoder_param,
            adc_param: motor_adc_param,
        })
    }

    pub fn get_motor_feature_params(&mut self) -> Result<MotorFeatureParams> {
        let mut poles = 0;
        let mut rs_ohm = 0.0;
        let mut ls_d = 0.0;
        let mut rated_flux = 0.0;
        let mut res_est_curr = 0.0;
        let mut ind_est_curr = 0.0;
        let mut max_curr = 0.0;
        let mut flux_exec_freq = 0.0;
        let mut wbp_kgm2 = 0.0;
        let mut rated_vol = 0.0;

        if let Some(buf) = self.request(GetCmdTypes::GetPolePairs as u8, 0) {
            if buf.len() >= 4 {
                poles = vec_to_int(&buf[0..4]) as u32;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetParamRs as u8, 0) {
            if buf.len() >= 4 {
                rs_ohm = vec_to_int(&buf[0..4]) as f32 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetParamLsD as u8, 0) {
            if buf.len() >= 4 {
                ls_d = vec_to_int(&buf[0..4]) as f32 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetParamFlux as u8, 0) {
            if buf.len() >= 4 {
                rated_flux = vec_to_int(&buf[0..4]) as f32 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetResEstCurrent as u8, 0) {
            if buf.len() >= 4 {
                res_est_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetIndEstCurrent as u8, 0) {
            if buf.len() >= 4 {
                ind_est_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetMaxCurrent as u8, 0) {
            if buf.len() >= 4 {
                max_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetFluxExecFreq as u8, 0) {
            if buf.len() >= 4 {
                flux_exec_freq = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetWbpKgm2 as u8, 0) {
            if buf.len() >= 4 {
                wbp_kgm2 = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetRatedVoltage as u8, 0) {
            if buf.len() >= 4 {
                rated_vol = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        Ok(MotorFeatureParams {
            poles: Some(poles),
            rs_ohm: Some(rs_ohm),
            ls_d: Some(ls_d),
            ls_q: Some(ls_d),
            rated_flux: Some(rated_flux),
            res_est_current: Some(res_est_curr),
            ind_est_current: Some(ind_est_curr),
            max_current: Some(max_curr),
            flux_exec_freq: Some(flux_exec_freq),
            wbp_kgm2: Some(wbp_kgm2),
            rated_voltage: Some(rated_vol),
        })
    }

    pub fn get_motor_startup_params(&mut self) -> Result<MotorStartupParams> {
        let mut flux_curr = 0.0;
        let mut align_curr = 0.0;
        let mut startup_curr = 0.0;
        let mut torque_curr = 0.0;
        let mut speed_start = 0.0;
        let mut speed_force = 0.0;

        if let Some(buf) = self.request(GetCmdTypes::GetFluxCurrent as u8, 0) {
            if buf.len() >= 4 {
                flux_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAlignCurrent as u8, 0) {
            if buf.len() >= 4 {
                align_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetStartupCurrent as u8, 0) {
            if buf.len() >= 4 {
                startup_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetTorqueCurrent as u8, 0) {
            if buf.len() >= 4 {
                torque_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetSpeedStart as u8, 0) {
            if buf.len() >= 4 {
                speed_start = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetSpeedForce as u8, 0) {
            if buf.len() >= 4 {
                speed_force = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        Ok(MotorStartupParams {
            flux_current: Some(flux_curr),
            align_current: Some(align_curr),
            startup_current: Some(startup_curr),
            torque_current: Some(torque_curr),
            speed_start: Some(speed_start),
            speed_force: Some(speed_force),
        })
    }

    pub fn get_motor_faultchk_params(&mut self) -> Result<MotorFaultChkParams> {
        let mut over_curr = 0.0;
        let mut over_vol = 0.0;
        let mut under_vol = 0.0;
        let mut over_load = 0.0;
        let mut stall_curr = 0.0;
        let mut falut_chk_curr = 0.0;
        let mut fail_spd_max = 0.0;
        let mut fail_spd_min = 0.0;

        if let Some(buf) = self.request(GetCmdTypes::GetOverCurrentFault as u8, 0) {
            if buf.len() >= 4 {
                over_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetOverVoltageFault as u8, 0) {
            if buf.len() >= 4 {
                over_vol = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetUnderVoltageFault as u8, 0) {
            if buf.len() >= 4 {
                under_vol = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetOverLoadPower as u8, 0) {
            if buf.len() >= 4 {
                over_load = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetStallCurrent as u8, 0) {
            if buf.len() >= 4 {
                stall_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetFaultCheckCurrent as u8, 0) {
            if buf.len() >= 4 {
                falut_chk_curr = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetFailSpeedMax as u8, 0) {
            if buf.len() >= 4 {
                fail_spd_max = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetFailSpeedMin as u8, 0) {
            if buf.len() >= 4 {
                fail_spd_min = vec_to_int(&buf[0..4]) as f32 / 1000.0;
            }
        }

        Ok(MotorFaultChkParams {
            over_current: Some(over_curr),
            over_voltage: Some(over_vol),
            under_voltage: Some(under_vol),
            over_load_power: Some(over_load),
            stall_current: Some(stall_curr),
            fault_ckeck_current: Some(falut_chk_curr),
            fail_speed_max: Some(fail_spd_max),
            fail_speed_min: Some(fail_spd_min),
        })
    }

    pub fn get_motor_encoder_params(&mut self) -> Result<MotorEncoderParams> {
        let mut encoder_slots = 0;

        if let Some(buf) = self.request(GetCmdTypes::GetEncSlots as u8, 0) {
            if buf.len() >= 4 {
                encoder_slots = vec_to_int(&buf[0..4]) as u32;
            }
        }

        Ok(MotorEncoderParams {
            slots: Some(encoder_slots),
        })
    }

    pub fn get_motor_adc_params(&mut self) -> Result<MotorAdcParams> {
        let mut vol_scale = 0.0;
        let mut cur_scale = 0.0;
        let mut vol_filter_pole = 0.0;
        let mut off_a_cur = 0.0;
        let mut off_b_cur = 0.0;
        let mut off_c_cur = 0.0;
        let mut off_a_vol = 0.0;
        let mut off_b_vol = 0.0;
        let mut off_c_vol = 0.0;

        if let Some(buf) = self.request(GetCmdTypes::GetAdcScaleVol as u8, 0) {
            if buf.len() >= 4 {
                vol_scale = vec_to_int(&buf[0..4]) as f32 / 1000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcScaleCur as u8, 0) {
            if buf.len() >= 4 {
                cur_scale = vec_to_int(&buf[0..4]) as f32 / 1000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcVolFilterPole as u8, 0) {
            if buf.len() >= 4 {
                vol_filter_pole = vec_to_int(&buf[0..4]) as f32 / 1000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcOffACurrent as u8, 0) {
            if buf.len() >= 4 {
                off_a_cur = vec_to_int(&buf[0..4]) as f32 / 10000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcOffBCurrent as u8, 0) {
            if buf.len() >= 4 {
                off_b_cur = vec_to_int(&buf[0..4]) as f32 / 10000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcOffCCurrent as u8, 0) {
            if buf.len() >= 4 {
                off_c_cur = vec_to_int(&buf[0..4]) as f32 / 10000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcOffAVoltage as u8, 0) {
            if buf.len() >= 4 {
                off_a_vol = vec_to_int(&buf[0..4]) as f32 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcOffBVoltage as u8, 0) {
            if buf.len() >= 4 {
                off_b_vol = vec_to_int(&buf[0..4]) as f32 / 100000000.0;
            }
        }

        if let Some(buf) = self.request(GetCmdTypes::GetAdcOffCVoltage as u8, 0) {
            if buf.len() >= 4 {
                off_c_vol = vec_to_int(&buf[0..4]) as f32 / 100000000.0;
            }
        }

        Ok(MotorAdcParams {
            vol_scale: Some(vol_scale),
            cur_scale: Some(cur_scale),
            vol_filter_pole: Some(vol_filter_pole),
            off_a_cur: Some(off_a_cur),
            off_b_cur: Some(off_b_cur),
            off_c_cur: Some(off_c_cur),
            off_a_vol: Some(off_a_vol),
            off_b_vol: Some(off_b_vol),
            off_c_vol: Some(off_c_vol),
        })
    }

    pub fn update_motor_speed_rps(&mut self, rps: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetMotorSpeedRps as u8, rps as i32) {}
        Ok(())
    }

    pub fn update_motor_speed_hz(&mut self, speed_hz: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetMotorSpeedHz as u8, speed_hz as i32) {}
        Ok(())
    }

    pub fn start_motor(&mut self) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetMotorStart as u8, 0) {}
        Ok(())
    }

    pub fn stop_motor(&mut self) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetMotorStop as u8, 0) {}
        Ok(())
    }

    pub fn reset_motor(&mut self) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetMotorReset as u8, 0) {}
        Ok(())
    }

    pub fn set_motor_identify_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetEnableIndentify as u8, en as i32) {}
        Ok(())
    }

    pub fn set_motor_rs_online_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetEnableRsOnline as u8, en as i32) {}
        Ok(())
    }

    pub fn set_motor_rs_recalc_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetEnableRsRecalc as u8, en as i32) {}
        Ok(())
    }

    pub fn clear_motor_faults(&mut self) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetClearFaults as u8, 0) {}
        Ok(())
    }

    pub fn update_motor_acc_max(&mut self, hz: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetAccMax as u8, hz as i32) {}
        Ok(())
    }

    pub fn update_motor_acc_start(&mut self, hz: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetAccStart as u8, hz as i32) {}
        Ok(())
    }

    pub fn update_motor_kp_spd(&mut self, kp: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetKpSpd as u8, kp as i32) {}
        Ok(())
    }

    pub fn update_motor_ki_spd(&mut self, ki: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetKiSpd as u8, ki as i32) {}
        Ok(())
    }

    pub fn update_motor_kp_iq(&mut self, kp: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetKpIq as u8, kp as i32) {}
        Ok(())
    }

    pub fn update_motor_ki_iq(&mut self, ki: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetKiIq as u8, ki as i32) {}
        Ok(())
    }

    pub fn set_motor_pos_ctrl_enable(&mut self, en: bool) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetEnablePosCtrl as u8, en as i32) {}
        Ok(())
    }

    pub fn update_motor_position(&mut self, pos: u32) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetMotorPosition as u8, pos as i32) {}
        Ok(())
    }

    fn request(&mut self, msg_type: u8, msg: i32) -> Option<Vec<u8>> {
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

        let crc_engine = crc::Crc::<u8>::new(&crc::CRC_8_CDMA2000);
        let crc = crc_engine.checksum(&cmd[2..8]);
        cmd.insert(cmd.len() - 2, crc);

        if let Some(ref mut port) = self.port {
            match port.write(&cmd) {
                core::result::Result::Ok(_) => {
                    // read msg
                    let mut buf: Vec<u8> = vec![0; 128];

                    match port.read(buf.as_mut_slice()) {
                        core::result::Result::Ok(t) => {
                            if t < 7 {
                                return None;
                            }

                            let len = buf[3] as usize;
                            let crc = buf[t - 3];

                            if crc != crc_engine.checksum(&buf[2..t - 3]) {
                                return None;
                            }

                            if buf[0] != 0x5a
                                || buf[1] != 0x5a
                                || buf[len + 7 - 1] != 0xa5
                                || buf[len + 7 - 2] != 0xa5
                            {
                                return None;
                            }

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

    pub fn update_motor_special_params(&mut self, param: MotorSpecialParams) -> Result<()> {
        self.update_motor_feature_params(param.feature_param)
            .unwrap();

        self.update_motor_startup_params(param.startup_param)
            .unwrap();

        self.update_motor_faultchk_params(param.fault_check_param)
            .unwrap();

        self.update_motor_encoder_params(param.encoder_param)
            .unwrap();

        self.update_motor_adc_params(param.adc_param).unwrap();

        Ok(())
    }

    pub fn update_motor_feature_params(&mut self, param: MotorFeatureParams) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetPolePairs as u8, param.poles.unwrap() as i32)
        {
        }

        if let Some(_) = self.request(
            SetCmdTypes::SetParamRs as u8,
            (param.rs_ohm.unwrap() * 100000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetParamLsD as u8,
            (param.ls_d.unwrap() * 100000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetParamLsQ as u8,
            (param.ls_d.unwrap() * 100000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetParamFlux as u8,
            (param.rated_flux.unwrap() * 100000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetResEstCurrent as u8,
            (param.res_est_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetIndEstCurrent as u8,
            (param.ind_est_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetMaxCurrent as u8,
            (param.max_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetFluxExecFreq as u8,
            (param.flux_exec_freq.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetWbpKgm2 as u8,
            (param.wbp_kgm2.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetRatedVoltage as u8,
            (param.rated_voltage.unwrap() * 1000.0) as i32,
        ) {}

        Ok(())
    }

    pub fn update_motor_startup_params(&mut self, param: MotorStartupParams) -> Result<()> {
        if let Some(_) = self.request(
            SetCmdTypes::SetFluxCurrent as u8,
            (param.flux_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAlignCurrent as u8,
            (param.align_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetStartupCurrent as u8,
            (param.startup_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetTorqueCurrent as u8,
            (param.torque_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetSpeedStart as u8,
            (param.speed_start.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetSpeedForce as u8,
            (param.speed_force.unwrap() * 1000.0) as i32,
        ) {}

        Ok(())
    }

    pub fn update_motor_faultchk_params(&mut self, param: MotorFaultChkParams) -> Result<()> {
        if let Some(_) = self.request(
            SetCmdTypes::SetOverCurrentFault as u8,
            (param.over_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetOverVoltageFault as u8,
            (param.over_voltage.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetUnderVoltageFault as u8,
            (param.under_voltage.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetOverLoadPower as u8,
            (param.over_load_power.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetStallCurrent as u8,
            (param.stall_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetFaultCheckCurrent as u8,
            (param.fault_ckeck_current.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetFailSpeedMax as u8,
            (param.fail_speed_max.unwrap() * 1000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetFailSpeedMin as u8,
            (param.fail_speed_min.unwrap() * 1000.0) as i32,
        ) {}

        Ok(())
    }

    pub fn update_motor_encoder_params(&mut self, param: MotorEncoderParams) -> Result<()> {
        if let Some(_) = self.request(SetCmdTypes::SetEncSlots as u8, param.slots.unwrap() as i32) {
        }

        Ok(())
    }

    pub fn update_motor_adc_params(&mut self, param: MotorAdcParams) -> Result<()> {
        if let Some(_) = self.request(
            SetCmdTypes::SetAdcScaleVol as u8,
            (param.vol_scale.unwrap() * 1000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcScaleCur as u8,
            (param.cur_scale.unwrap() * 1000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcVolFilterPole as u8,
            (param.vol_filter_pole.unwrap() * 1000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcOffACurrent as u8,
            (param.off_a_cur.unwrap() * 10000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcOffBCurrent as u8,
            (param.off_b_cur.unwrap() * 10000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcOffCCurrent as u8,
            (param.off_c_cur.unwrap() * 10000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcOffAVoltage as u8,
            (param.off_a_vol.unwrap() * 100000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcOffBVoltage as u8,
            (param.off_b_vol.unwrap() * 100000000.0) as i32,
        ) {}

        if let Some(_) = self.request(
            SetCmdTypes::SetAdcOffCVoltage as u8,
            (param.off_c_vol.unwrap() * 100000000.0) as i32,
        ) {}

        Ok(())
    }

    pub fn export_motor_special_params(
        &mut self,
        param: MotorSpecialParams,
        path: String,
    ) -> Result<()> {
        let yaml = serde_yaml::to_string(&param).unwrap();
        tools::save_yaml(yaml, path.as_str())?;
        Ok(())
    }

    pub fn import_motor_special_params(&mut self, path: String) -> Result<Value> {
        if !tools::is_file_exist(&path) {
            bail!("File: {} not exist", path)
        }

        match tools::read_yaml::<Value>(&path) {
            core::result::Result::Ok(param) => Ok(param),
            Err(err) => {
                bail!("Failed to read profile: {}", err)
            }
        }
    }
}

#[allow(dead_code)]
pub fn vec_to_long(buf: &[u8]) -> i64 {
    ((buf[0] as i64) << 56
        | (buf[1] as i64) << 48
        | (buf[2] as i64) << 40
        | (buf[3] as i64) << 32
        | (buf[4] as i64) << 24
        | (buf[5] as i64) << 16
        | (buf[6] as i64) << 8
        | (buf[7] as i64)) as i64
}

pub fn vec_to_int(buf: &[u8]) -> i32 {
    ((buf[0] as i32) << 24 | (buf[1] as i32) << 16 | (buf[2] as i32) << 8 | (buf[3] as i32)) as i32
}

pub fn vec_to_short(buf: &[u8]) -> i16 {
    ((buf[0] as i16) << 8 | (buf[1] as i16)) as i16
}
