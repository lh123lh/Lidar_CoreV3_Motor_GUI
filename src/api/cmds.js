import { ref, watch } from 'vue'
import { ElNotification } from 'element-plus'
import { invoke } from '@tauri-apps/api/tauri'

function cmd_connect_motor(sp, baud) {
  return new Promise(function (resolve, reject) {
    invoke('init_serial_port', { sp: sp, baud: baud })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        import_failed(error)
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
        import_failed(error)
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
        import_failed(error)
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
        import_failed(error)
        resolve()
      })
  })
}

function import_failed(msg) {
  ElNotification({
    title: 'Error',
    message: msg,
    type: 'error',
    showClose: false,
    duration: 500,
  })
}

export default {
  cmd_connect_motor,
  cmd_disconnect_motor,
  cmd_get_motor_params,
  cmd_get_avaliable_ports,
}