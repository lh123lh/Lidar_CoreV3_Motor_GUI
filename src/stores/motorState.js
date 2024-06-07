import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useMotorStore = defineStore('motor', () => {
  const currRps = ref(0.0);
  const currPos = ref(0.0);
  const isTesting = ref(false);
  const isConnected = ref(false);

  return { currRps, currPos, isTesting, isConnected };
})

export const MotorParams = defineStore('params', () => {
  const feature_param = ref({
    poles: 0,
    rs_ohm: 0.0,
    ls_d: 0.0,
    ls_q: 0.0,
    rated_flux: 0.0,
    res_est_current: 0.0,
    ind_est_current: 0.0,
    max_current: 0.0,
    flux_exec_freq: 0.0,
    wbp_kgm2: 0.0,
    rated_voltage: 0.0,
  });

  const startup_param = ref({
    flux_current: 0.0,
    align_current: 0.0,
    startup_current: 0.0,
    torque_current: 0.0,
    speed_start: 0.0,
    speed_force: 0.0,
  });

  const fault_check_param = ref({
    over_current: 0.0,
    over_voltage: 0.0,
    under_voltage: 0.0,
    over_load_power: 0.0,
    stall_current: 0.0,
    fault_ckeck_current: 0.0,
    fail_speed_max: 0.0,
    fail_speed_min: 0.0,
  });

  const encoder_param = ref({
    slots: 0,
  });

  const adc_param = ref({
    vol_scale: 0.0,
    cur_scale: 0.0,
    vol_filter_pole: 0.0,
    off_a_cur: 0.0,
    off_b_cur: 0.0,
    off_c_cur: 0.0,
    off_a_vol: 0.0,
    off_b_vol: 0.0,
    off_c_vol: 0.0,
  });

  return { feature_param, startup_param, fault_check_param, encoder_param, adc_param };
})

// export const useMotorStore = defineStore('motor', {
//   state: () => ({ currRps: 0.0 }),
//   getters: {
//     get: (state) => state.currRps,
//   },
//   actions: {
//     update(rps) {
//       this.$state.currRps = rps;
//     }
//   },
// })
