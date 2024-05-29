import { ref, watch } from 'vue'
import { ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/tauri'

function notify_failed(msg) {
  ElNotification({
    title: 'Error',
    message: msg,
    type: 'error',
    showClose: false,
    duration: 500,
  })
}

function formatSeconds(sec) {
  const date = new Date(sec * 1000);
  const hours = date.getUTCHours();
  const minutes = date.getUTCMinutes();
  const seconds = date.getUTCSeconds();

  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
}

function cmd_connect_motor(sp, baud) {
  return new Promise(function (resolve, reject) {
    invoke('init_serial_port', { sp: sp, baud: baud })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
      })
  })
}

function cmd_disconnect_motor(sp, baud) {
  return new Promise(function (resolve, reject) {
    invoke('deinit_serial_port', {})
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
      })
  })
}

function cmd_get_motor_current_rps() {
  return new Promise(function (resolve, reject) {
    invoke('get_motor_current_rps', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  }).catch(err => {
    this.$message.error(err.message);
    console.log(err);
  });
}

function cmd_get_motor_params() {
  return new Promise(function (resolve, reject) {
    invoke('get_motor_params', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_get_motor_static_params() {
  return new Promise(function (resolve, reject) {
    invoke('get_motor_static_params', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_get_motor_status() {
  return new Promise(function (resolve, reject) {
    invoke('get_motor_status', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_get_motor_special_params() {
  return new Promise(function (resolve, reject) {
    invoke('get_motor_special_params', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_get_avaliable_ports() {
  return new Promise(function (resolve, reject) {
    invoke('list_avaliable_ports', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_motor_rps(rps) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_rps', { rps: rps })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_enable_motor_identify(enable) {
  return new Promise(function (resolve, reject) {
    invoke('enable_motor_identify', { en: enable })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_enable_motor_rs_online(enable) {
  return new Promise(function (resolve, reject) {
    invoke('enable_motor_rs_online', { en: enable })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_enable_motor_rs_recalc(enable) {
  return new Promise(function (resolve, reject) {
    invoke('enable_motor_rs_recalc', { en: enable })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_clear_motor_faults(enable) {
  return new Promise(function (resolve, reject) {
    invoke('clear_motor_faults', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_start_motor(rps) {
  return new Promise(function (resolve, reject) {
    invoke('start_motor', { rps: rps })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_stop_motor() {
  return new Promise(function (resolve, reject) {
    invoke('stop_motor', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_acc_max(hz) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_acc_max', { hz: hz })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_acc_start(hz) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_acc_start', { hz: hz })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_kp_spd(kp) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_kp_spd', { kp: kp })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_ki_spd(ki) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_ki_spd', { ki: ki })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_kp_iq(kp) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_kp_iq', { kp: kp })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_ki_iq(ki) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_ki_ip', { ki: ki })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_enable_motor_pos_ctrl(enable) {
  return new Promise(function (resolve, reject) {
    invoke('enable_motor_pos_ctrl', { en: enable })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_update_motor_position(pos) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_position', { pos: pos })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_start_record_rps(path) {
  if (path === null || path === undefined || path.trim() === '') {
    notify_failed("record path is empty!");

    return;
  }

  return new Promise(function (resolve, reject) {
    invoke('start_record_rps', { path: path })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_stop_record_rps() {
  return new Promise(function (resolve, reject) {
    invoke('stop_record_rps', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_start_startup_test(rps, count, cold_duration) {
  return new Promise(function (resolve, reject) {
    invoke('start_startup_task', {rps: rps, count: count, cold: cold_duration})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_stop_startup_test() {
  return new Promise(function (resolve, reject) {
    invoke('stop_startup_task', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

function cmd_get_startup_test_result() {
  return new Promise(function (resolve, reject) {
    invoke('get_startup_test_result', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
        resolve()
      })
  })
}

export default {
  notify_failed,
  formatSeconds,
  cmd_connect_motor,
  cmd_disconnect_motor,
  cmd_get_motor_current_rps,
  cmd_get_motor_params,
  cmd_get_motor_status,
  cmd_get_motor_static_params,
  cmd_get_motor_special_params,
  cmd_get_avaliable_ports,
  cmd_update_motor_rps,
  cmd_enable_motor_identify,
  cmd_enable_motor_rs_online,
  cmd_enable_motor_rs_recalc,
  cmd_clear_motor_faults,
  cmd_start_motor,
  cmd_stop_motor,
  cmd_update_acc_max,
  cmd_update_acc_start,
  cmd_update_kp_spd,
  cmd_update_ki_spd,
  cmd_update_kp_iq,
  cmd_update_ki_iq,
  cmd_enable_motor_pos_ctrl,
  cmd_update_motor_position,
  cmd_start_record_rps,
  cmd_stop_record_rps,
  cmd_start_startup_test,
  cmd_stop_startup_test,
  cmd_get_startup_test_result,
}