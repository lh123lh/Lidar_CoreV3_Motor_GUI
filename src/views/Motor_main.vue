<script setup>
import { ref, onMounted, nextTick, computed, watch, onActivated, onDeactivated } from "vue";
import cmds from '../utils/cmds';
import cardBase from "../components/cardBase.vue";
import UploadDialog from "../components/UploadDialog.vue";
import PageBase from "../components/PageBase.vue";
import { useMotorStore } from '../stores/motorState'
import parseErrorCode from "../utils/parseErrorCode.js";
import { useI18n } from 'vue-i18n';
import SpeedMeter from '../components/SpeedMeter.vue';
import MotorPosGuage from "../components/MotorPosGuage.vue";

const { t } = useI18n()

const store = useMotorStore()
const Rs_Ohm = ref(0.00);
const Rs_Ohm_Online = ref(0.00);
const Ls_d = ref(0.00);
const Ls_q = ref(0.00);
const flux = ref(0.00);
const poles = ref(0); //  磁极对数
const connectBtn = ref(t('main.connect'));
const startBtn = ref(t('main.start'));
const targetRps = ref(0.00);
const currentRps = ref(0.00);
const currentPos = ref(0.0);
const vdcBus = ref(0.0);
const enableRsOnline = ref(false);
const enableRsReCalc = ref(false);
const motorIdentified = ref(false);
const motorStarted = ref(false);
const updateDialogVisible = ref(false);
const updateStatus = ref(false);
const motorState = ref("STOP_IDLE");
const mctrlState = ref("FIRST_RUN");
const errorCode = ref(0);
const isActived = ref(false);

const accMaxHz = ref(0.0);
const accStartHz = ref(0.0);
const kp_spd = ref(0.0);
const ki_spd = ref(0.0);
const kp_iq = ref(0.0);
const ki_iq = ref(0.0);
const motor_version = ref("v0.0.0a_20240101")

const baudRates = [
  {
    value: 9600,
    label: '9600',
  },
  {
    value: 38400,
    label: '38400',
  },
  {
    value: 115200,
    label: '115200',
  },
  {
    value: 230400,
    label: '230400',
  },
  {
    value: 460800,
    label: '460800',
  },
]
const baudRate = ref(115200);
const serialPorts = ref(["COM0"]);
const serialPort = ref();

const errorCodeForm = ref([])

const ctrlModes = [
  {
    value: 0,
    label: t('main.speedMode'),
  },
  {
    value: 1,
    label: t('main.posMode'),
  }
]
const ctrlMode = ref(0);
const targetPosition = ref(0.0);
const currentPosition = ref(0.0);

onMounted(() => { //组件挂载时的生命周期执行的方法
  get_avaliable_ports()

  window.setInterval(function timer() {
    if (store.isConnected && !store.isTesting) {
      get_motor_runtime_params();
      get_motor_status();
    }
  }, 1000);

  window.setInterval(function timer() {
    if (store.isConnected && !store.isTesting) {
      if (ctrlMode.value == 0)
        get_motor_current_rps()
      else
        get_motor_current_pos()
    }
  }, 500);

  window.setInterval(function timer() {
    get_avaliable_ports()
  }, 3000);
})

onActivated(() => {
  isActived.value = true;
})

onDeactivated(() => {
  isActived.value = false;
})

watch(enableRsOnline, (newVal, oldVal) => {
  enable_motor_rs_online(enableRsOnline.value);
})

watch(enableRsReCalc, (newVal, oldVal) => {
  enable_motor_rs_recalc(enableRsReCalc.value);
})

async function connect_motor() {
  if (store.isConnected) {
    store.isConnected = false;
    motorStarted.value = false;
    startBtn.value = t('main.start');
    connectBtn.value = t('main.connect');
    await cmds.cmd_disconnect_motor(serialPort.value, baudRate.value);
  } else {
    store.isConnected = true;
    connectBtn.value = t('main.disconnect');
    await cmds.cmd_connect_motor(serialPort.value, baudRate.value);

    get_motor_static_params();
  }
}

// 获取电机启动参数, 运行时不可变
async function get_motor_static_params() {
  await cmds.cmd_get_motor_static_params()
    .then((data) => {
      Rs_Ohm.value = data.rs;
      Rs_Ohm_Online.value = data.rs_online;
      Ls_d.value = data.ls_d;
      Ls_q.value = data.ls_q;
      flux.value = data.flux;
      poles.value = data.poles;
      accMaxHz.value = data.acc_max_hzps;
      accStartHz.value = data.acc_start_hzps;
      kp_spd.value = data.kp_spd;
      ki_spd.value = data.ki_spd;
      kp_iq.value = data.kp_iq;
      ki_iq.value = data.ki_iq;
      motor_version.value = `v${data.main_version}.${data.sub_version}.${data.rev_version}${data.stage_version}_${data.version_date}`;
    })
}

// 获取电机运行时参数
async function get_motor_runtime_params() {
  await cmds.cmd_get_motor_params()
    .then((data) => {
      vdcBus.value = data.vdc_bus;
    })
}

async function get_motor_current_rps() {
  await cmds.cmd_get_motor_current_rps()
    .then((data) => {
      store.currRps = parseFloat(data.toFixed(5));
      // store.update(parseFloat(data.toFixed(3)));
      if (store.currRps <= -1000.0 && store.isConnected) {
        cmds.notify_failed("电机已断开连接");
        connect_motor();
      } else {
        if (isActived.value) {
          currentRps.value = store.currRps;
        }
      }
    }).catch(err => {
      this.$message.error(err.message);
      console.log(err);
    });
}

async function get_motor_current_pos() {
  await cmds.cmd_get_motor_current_pos()
    .then((data) => {
      store.currPos = parseFloat(data.toFixed(3));
      // store.update(parseFloat(data.toFixed(3)));
      if (store.currPos <= -1000.0) {
        cmds.notify_failed("电机已断开连接");
        connect_motor();
      } else {
        if (isActived.value) {
          currentPos.value = store.currPos;
        }
      }
    }).catch(err => {
      this.$message.error(err.message);
      console.log(err);
    });
}

async function get_motor_status() {
  await cmds.cmd_get_motor_status()
    .then((data) => {
      motorIdentified.value = data.identified;
      errorCode.value = data.error_code;
      motorState.value = data.motor_state;
      mctrlState.value = data.mctrl_state;
      enableRsOnline.value = data.rsonline_en;
      enableRsReCalc.value = data.rsrecalc_en;

      errorCodeForm.value = parseErrorCode.getFaultStates(errorCode.value);
    })
}

async function get_avaliable_ports() {
  await cmds.cmd_get_avaliable_ports()
    .then((data) => {
      serialPorts.value = data;

      // 在setInterval中定时查询可用端口时需要去掉下面的代码
      // serialPort.value = serialPorts.value[0];
    })
}

async function update_motor_rps() {
  if (motorStarted.value) {
    await cmds.cmd_update_motor_rps(parseFloat(targetRps.value))
  } else {
    if (targetRps.value == 0) {
      cmds.notify_failed("请先设置目标转速");
      return;
    }

    motorStarted.value = true;
    startBtn.value = t('main.update');
    await cmds.cmd_start_motor(parseFloat(targetRps.value))
  }
}

async function update_motor_positon() {
  if (motorStarted.value) {
    await cmds.cmd_update_motor_position(parseFloat(targetPosition.value))
  } else {
    motorStarted.value = true;
    startBtn.value = "更新";
    await cmds.cmd_enable_motor_pos_ctrl(true);
    await cmds.cmd_start_motor(parseFloat(10.0));
  }
}

async function stop_motor() {
  await cmds.cmd_stop_motor()
    .then((data) => {
      motorStarted.value = false;
      startBtn.value = t('main.start');
    })

  await cmds.cmd_enable_motor_pos_ctrl(false);
}

async function enable_motor_identify(enable) {
  await cmds.cmd_enable_motor_identify(enable)
    .then((data) => {
    })
}

async function enable_motor_rs_online(enable) {
  await cmds.cmd_enable_motor_rs_online(enable)
    .then((data) => {
    })
}

async function enable_motor_rs_recalc(enable) {
  await cmds.cmd_enable_motor_rs_recalc(enable)
    .then((data) => {
    })
}

async function clear_motor_faults() {
  await cmds.cmd_clear_motor_faults()
    .then((data) => {
    })
}

async function update_acc_max() {
  await cmds.cmd_update_acc_max(parseFloat(accMaxHz.value))
    .then((data) => {
    })
}

async function update_acc_start() {
  await cmds.cmd_update_acc_start(parseFloat(accStartHz.value))
    .then((data) => {
    })
}

async function update_kp_spd() {
  await cmds.cmd_update_kp_spd(parseFloat(kp_spd.value))
    .then((data) => {
    })
}

async function update_ki_spd() {
  await cmds.cmd_update_ki_spd(parseFloat(ki_spd.value))
    .then((data) => {
    })
}

async function update_kp_iq() {
  await cmds.cmd_update_kp_iq(parseFloat(kp_iq.value))
    .then((data) => {
    })
}

async function update_ki_iq() {
  await cmds.cmd_update_ki_iq(parseFloat(ki_iq.value))
    .then((data) => {
    })
}

async function upgrade_motor_fw(path) {
  console.log(path, serialPort.value, baudRate.value)
  updateStatus.value = true;
  await cmds.cmd_upgrade_motor_fw(path, serialPort.value, baudRate.value)
    .then(() => {
      updateDialogVisible.value = false;
      cmds.notify_success("升级完成");
      updateStatus.value = false;
    })
    .catch(() => {
      cmds.notify_failed("升级失败");
      updateStatus.value = false;
    })
}

</script>

<template>
  <PageBase :title="$t('main.motorCfg')">
    <el-row :gutter="5">
      <el-col :span="12">
        <cardBase :title="$t('main.motorCfg')">
          <template #content>
            <el-row :gutter="5" class="mt-1">
              <el-col>
                <el-row>
                  <el-col :span="5">
                    <label>{{ $t('main.serialPort') }}</label>
                  </el-col>
                  <el-col :span="10">
                    <el-select v-model="serialPort" placeholder="Serial Port" :disabled=store.isConnected>
                      <el-option v-for="item in serialPorts" :value="item" />
                    </el-select>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="5">
                    <label>{{ $t('main.baudRate') }}</label>
                  </el-col>
                  <el-col :span="10">
                    <el-select v-model="baudRate" placeholder="Baud Rate" :disabled=store.isConnected>
                      <el-option v-for="item in baudRates" :key="item.value" :label="item.label" :value="item.value" />
                    </el-select>
                  </el-col>
                  <el-col :span="4">
                    <el-button @click="connect_motor" type="primary" plain>{{ connectBtn }}</el-button>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="5">
                    <label>{{ $t('main.ctrlMode') }}</label>
                  </el-col>
                  <el-col :span="10">
                    <el-select v-model="ctrlMode" placeholder="Ctrl Mode" :disabled=store.isConnected>
                      <el-option v-for="item in ctrlModes" :key="item.value" :label="item.label" :value="item.value" />
                    </el-select>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-0">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="15">
                    <label>{{ $t('main.rsOnline') }}</label>
                  </el-col>
                  <el-col :span="4">
                    <el-switch v-model="enableRsOnline" :disabled=!store.isConnected />
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-0">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="15">
                    <label>{{ $t('main.rsReCalc') }}</label>
                  </el-col>
                  <el-col :span="4">
                    <el-switch v-model="enableRsReCalc" :disabled=!store.isConnected />
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col>
                <el-row :gutter="5" v-if="ctrlMode == 0">
                  <el-col :span="5">
                    <label>{{ $t('main.targetRps') }}</label>
                  </el-col>
                  <el-col :span="10">
                    <el-input v-model="targetRps" :disabled=!store.isConnected>
                      <template #append>rps</template>
                    </el-input>
                  </el-col>

                  <el-col :span="9">
                    <el-row :gutter="5">
                      <el-col :span="12">
                        <el-button @click="update_motor_rps" :disabled=!store.isConnected type="primary" plain>{{
                          startBtn
                          }}</el-button>
                      </el-col>
                      <el-col :span="12">
                        <el-button @click="stop_motor" :disabled=!motorStarted type="warning" plain>{{
                          $t('main.stop')
                          }}</el-button>
                      </el-col>
                    </el-row>

                  </el-col>

                </el-row>
                <el-row :gutter="5" v-else>
                  <el-col :span="5">
                    <label>{{ $t('main.targetPos') }}</label>
                  </el-col>
                  <el-col :span="10">
                    <el-input v-model="targetPosition" :disabled=!store.isConnected>
                      <template #append>&deg;</template>
                    </el-input>
                  </el-col>
                  <el-col :span="9">
                    <el-row :gutter="5">
                      <el-col :span="12">
                        <el-button @click="update_motor_positon" :disabled=!store.isConnected type="primary" plain>{{
                          startBtn
                          }}</el-button>
                      </el-col>

                      <el-col :span="12">
                        <el-button @click="stop_motor" :disabled=!motorStarted type="warning" plain>{{
                          $t('main.stop')
                          }}</el-button>
                      </el-col>
                    </el-row>

                  </el-col>
                </el-row>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase v-if="ctrlMode == 0" :title="$t('main.realTimeSpd')" class="mt-1">
          <template #content>
            <div>
              <SpeedMeter :value="currentRps" height="38vh" />
            </div>
          </template>
        </cardBase>
        <cardBase v-else :title="$t('main.realTimePos')" class="mt-1">
          <template #content>
            <div>
              <MotorPosGuage :value="currentPos" height="38vh" />
            </div>
          </template>
        </cardBase>

      </el-col>

      <el-col :span="12">
        <cardBase :title="$t('main.status')">
          <template #content>
            <el-scrollbar height="80.5vh" class="mt-1">
              <el-row :gutter="20">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.identifiedStatus') }}</label>
                    </el-col>
                    <el-col :span="14">
                      <div
                        :class="['status-indicator', { 'connected': motorIdentified, 'disconnected': !motorIdentified }]">
                      </div>
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.motorStatus') }}</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="motorState" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.mcStatus') }}</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="mctrlState" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.errorCode') }}</label>
                    </el-col>

                    <el-col :span="8">
                      <el-popover placement="right" :width="400" trigger="click">
                        <template #reference>
                          <el-input v-model="errorCode" :readonly=true />
                        </template>
                        <el-table :data="errorCodeForm">
                          <el-table-column width="500" property="flag" label="故障详情" />
                        </el-table>
                      </el-popover>

                    </el-col>
                    <el-col :span="6" style="text-align: end;">
                      <el-button @click="clear_motor_faults" :disabled=!store.isConnected type="danger"
                        plain>{{ $t('main.clearFaults') }}</el-button>
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Rs (Ohm)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="Rs_Ohm" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>
              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Ls-d (mH)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="Ls_d" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>
              <el-row class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label style="font-size: 1rem;">Ls-q (mH)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="Ls_q" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Flux (V/Hz)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="flux" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.magPoles') }}</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="poles" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.busVol') }} (V)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="vdcBus" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.maxAcc') }} (rps)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="accMaxHz" @input="update_acc_max" :readonly=!store.isConnected />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.startAcc') }} (rps)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="accStartHz" @input="update_acc_start" :readonly=!store.isConnected />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Kp SPD</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="kp_spd" @input="update_kp_spd" :readonly=!store.isConnected />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Ki SPD</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="ki_spd" @input="update_ki_spd" :readonly=!store.isConnected />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Kp Iq</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="kp_iq" @input="update_kp_iq" :readonly=!store.isConnected />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>Ki Iq</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="ki_iq" @input="update_ki_iq" :readonly=!store.isConnected />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1 mb-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>{{ $t('main.fwVersion') }}</label>
                    </el-col>
                    <el-col :span="8">
                      {{ motor_version }}
                    </el-col>
                    <el-col :span="6" style="text-align: end;">
                      <el-button @click="updateDialogVisible = true" :disabled=store.isConnected type="primary"
                        plain>{{ $t('main.upgrade') }}</el-button>
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

            </el-scrollbar>
          </template>
        </cardBase>
      </el-col>
    </el-row>
  </PageBase>

  <UploadDialog v-model="updateDialogVisible" :handleUpload="upgrade_motor_fw" :title="$t('fwUpgrade')" :uploadBtnName="$t('update')"
    :status="updateStatus" />

</template>

<style scoped>
.status-indicator {
  width: 15px;
  height: 15px;
  border-radius: 50%;
  display: inline-block;
}

.connected {
  background-color: #67C23A;
}

.disconnected {
  background-color: #F56C6C;
}

@media (min-width: 1200px) {
  .el-scrollbar {
    height: 85.5vh;
  }
}
</style>

<style lang="scss" scoped>
::v-deep(.is-horizontal) {
  height: 0px;
  left: 0px;
  display: none;
}

::v-deep(.el-scrollbar__wrap) {
  overflow-x: hidden;
}
</style>
