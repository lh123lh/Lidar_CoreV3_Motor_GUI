<script setup>
import { ref, onMounted, nextTick, computed, watch, onActivated, onDeactivated } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import cmds from '../api/cmds';
import cardBase from "../components/cardBase.vue";
import VueSpeedometer from "vue-speedometer";
import UpdataFwDialog from "../components/UpdataFwDialog.vue";
import PageBase from "../components/PageBase.vue";
import { useMotorStore } from '../stores/motorState'
import parseErrorCode from "../api/parseErrorCode.js";

const store = useMotorStore()
const Rs_Ohm = ref(0.00);
const Rs_Ohm_Online = ref(0.00);
const Ls_d = ref(0.00);
const Ls_q = ref(0.00);
const flux = ref(0.00);
const poles = ref(0); //  磁极对数
const connectBtn = ref("连接");
const startBtn = ref("启动");
const targetRps = ref(0.00);
const currentRps = ref(0.00);
const vdcBus = ref(0.0);
const enableIdentify = ref(false);
const enableRsOnline = ref(false);
const enableRsReCalc = ref(false);
const motorIdentified = ref(false);
const motorStarted = ref(false);
const updateDialogVisible = ref(false);
const motorState = ref("STOP_IDLE");
const mctrlState = ref("FIRST_RUN");
const errorCode = ref(0);
const isActived = ref(false);

const accMaxHz = ref(0.0);
const accStartHz = ref(0.0);

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
      get_motor_current_rps()
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

watch(enableIdentify, (newVal, oldVal) => {
  enable_motor_identify(enableIdentify.value);
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
    startBtn.value = "启动";
    connectBtn.value = "连接";
    await cmds.cmd_disconnect_motor(serialPort.value, baudRate.value);
  } else {
    store.isConnected = true;
    connectBtn.value = "断开";
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
      store.currRps = parseFloat(data.toFixed(3));
      // store.update(parseFloat(data.toFixed(3)));
      if (isActived.value) {
        currentRps.value = store.currRps;
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
      enableIdentify.value = data.identified_en;
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
      serialPort.value = serialPorts.value[0];
    })
}

async function update_motor_rps() {
  if (motorStarted.value) {
    await cmds.cmd_update_motor_rps(parseFloat(targetRps.value))
  } else {
    motorStarted.value = true;
    startBtn.value = "更新";
    await cmds.cmd_start_motor(parseFloat(targetRps.value))
  }

}

async function stop_motor() {
  await cmds.cmd_stop_motor()
    .then((data) => {
      motorStarted.value = false;
      startBtn.value = "启动";
    })
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

</script>

<template>
  <PageBase title="电机控制">
    <el-row :gutter="5">
      <el-col :span="12">
        <cardBase title="电机配置">
          <template #content>
            <el-row :gutter="5" class="mt-n3">
              <el-col>
                <el-row>
                  <el-col :span="5">
                    <label>串口</label>
                  </el-col>
                  <el-col :span="10">
                    <el-select v-model="serialPort" placeholder="Serial Port">
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
                    <label>波特率</label>
                  </el-col>
                  <el-col :span="10">
                    <el-select v-model="baudRate" placeholder="Baud Rate">
                      <el-option v-for="item in baudRates" :key="item.value" :label="item.label" :value="item.value" />
                    </el-select>
                  </el-col>
                  <el-col :span="4">
                    <el-button @click="connect_motor" type="primary" plain>{{ connectBtn }}</el-button>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-0">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="15">
                    <label>自动识别参数</label>
                  </el-col>
                  <el-col :span="4">
                    <el-switch v-model="enableIdentify" :disabled=!store.isConnected />
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-0">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="15">
                    <label>Rs在线估算</label>
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
                    <label>Rs重校准</label>
                  </el-col>
                  <el-col :span="4">
                    <el-switch v-model="enableRsReCalc" :disabled=!store.isConnected />
                  </el-col>
                </el-row>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="5">
                    <label>目标转速</label>
                  </el-col>
                  <el-col :span="10">
                    <el-input v-model="targetRps" :disabled=!store.isConnected>
                      <template #append>rps</template>
                    </el-input>
                  </el-col>
                  <el-col :span="4">
                    <el-button @click="update_motor_rps" :disabled=!store.isConnected type="primary" plain>{{ startBtn
                      }}</el-button>
                  </el-col>

                  <el-col :span="4" class="ms-1">
                    <el-button @click="stop_motor" :disabled=!motorStarted type="warning" plain>停止</el-button>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase title="实时转速" class="mt-1">
          <template #content>
            <VueSpeedometer :height="180" :value="currentRps" :minValue="0" :maxValue="250" :segments="10" />
          </template>
        </cardBase>

      </el-col>

      <el-col :span="12">
        <cardBase title="状态及参数">
          <template #content>
            <el-scrollbar max-height="29.0rem" class="mt-n3">
              <el-row :gutter="20">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label>电机识别状态</label>
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
                      <label>电机状态</label>
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
                      <label>MC状态</label>
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
                      <label>故障码</label>
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
                    <el-col :span="6">
                      <el-button @click="clear_motor_faults" :disabled=!store.isConnected type="danger" plain>清除错误</el-button>
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
                      <label>磁极对数</label>
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
                      <label>母线电压 (V)</label>
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
                      <label>最大加速度 (Hz)</label>
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
                      <label>启动加速度 (Hz)</label>
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
                      <label>固件版本</label>
                    </el-col>
                    <el-col :span="8">
                      v0.0.1_20240401
                    </el-col>
                    <el-col :span="6">
                      <el-button @click="updateDialogVisible = true" :disabled=!store.isConnected type="primary"
                        plain>升级固件</el-button>
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

  <UpdataFwDialog v-model="updateDialogVisible" />

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
