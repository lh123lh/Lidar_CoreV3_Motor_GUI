import { ref, watch } from 'vue'
import { ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/tauri'

function notify_success(msg) {
  ElNotification({
    title: 'Info',
    message: msg,
    type: 'success',
    showClose: true,
    duration: 1500,
  })
}

function notify_failed(msg) {
  ElNotification({
    title: 'Error',
    message: msg,
    type: 'error',
    showClose: true,
    duration: 1500,
  })
}

function notify_warning(msg) {
  ElNotification({
    title: 'Warning',
    message: msg,
    type: 'warning',
    showClose: true,
    duration: 1500,
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

function cmd_get_motor_current_pos() {
  return new Promise(function (resolve, reject) {
    invoke('get_motor_current_pos', {})
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

function cmd_enable_motor_pos_ctrl(enable, mode) {
  return new Promise(function (resolve, reject) {
    invoke('enable_motor_pos_ctrl', { en: enable, mode: mode })
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

function cmd_start_startup_test(test_param) {
  return new Promise(function (resolve, reject) {
    invoke('start_startup_task', { testParam: test_param })
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

function cmd_update_motor_special_params(param) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_special_params', { param: param })
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

function cmd_export_motor_special_params(param, path) {
  return new Promise(function (resolve, reject) {
    invoke('export_motor_special_params', { param: param, path: path })
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

function cmd_import_motor_special_params(path) {
  return new Promise(function (resolve, reject) {
    invoke('import_motor_special_params', { path: path })
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

function cmd_upgrade_motor_fw(path, sp, baud) {
  return new Promise(function (resolve, reject) {
    invoke('upgrade_motor_fw', { path: path, sp: sp, baud: baud })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        notify_failed(error)
        reject(error)
      })
  })
}

function cmd_merge_motor_fw(boot, app, output) {
  return new Promise(function (resolve, reject) {
    invoke('merge_firmware', { boot: boot, app: app, output: output })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        notify_failed(error)
        reject(error)
      })
  })
}

function cmd_connect_relay(sp, baud) {
  return new Promise(function (resolve, reject) {
    invoke('init_relay_port', { sp: sp, baud: baud })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
      })
  })
}

function cmd_disconnect_relay() {
  return new Promise(function (resolve, reject) {
    invoke('deinit_relay_port', {})
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
      })
  })
}

function cmd_set_relay_power(on) {
  return new Promise(function (resolve, reject) {
    invoke('set_relay_power', { on: on })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        notify_failed(error)
      })
  })
}

export default {
  notify_success,
  notify_failed,
  notify_warning,
  formatSeconds,
  cmd_connect_motor,
  cmd_disconnect_motor,
  cmd_get_motor_current_rps,
  cmd_get_motor_current_pos,
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
  cmd_update_motor_special_params,
  cmd_export_motor_special_params,
  cmd_import_motor_special_params,
  cmd_upgrade_motor_fw,
  cmd_merge_motor_fw,
  cmd_connect_relay,
  cmd_disconnect_relay,
  cmd_set_relay_power,
}