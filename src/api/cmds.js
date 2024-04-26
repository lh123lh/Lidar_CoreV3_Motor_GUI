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

function cmd_update_motor_rps(rps, poles) {
  return new Promise(function (resolve, reject) {
    invoke('update_motor_rps', { rps: rps, poles: poles })
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

function cmd_start_motor(rps, poles) {
  return new Promise(function (resolve, reject) {
    invoke('start_motor', { rps: rps, poles: poles })
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
    invoke('update_motor_acc_max', {hz: hz})
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
    invoke('update_motor_acc_start', {hz: hz})
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
  cmd_connect_motor,
  cmd_disconnect_motor,
  cmd_get_motor_params,
  cmd_get_motor_status,
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
}