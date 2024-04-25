<script setup>
import { ref, onMounted, nextTick, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import cmds from '../api/cmds';
import cardBase from "../components/cardBase.vue";
import VueSpeedometer from "vue-speedometer"

const isConnect = ref(false);
const Rs_Ohm = ref(0.00);
const Rs_Ohm_Online = ref(0.00);
const Ls_d = ref(0.00);
const Ls_q = ref(0.00);
const flux = ref(0.00);
const poles = ref(0); //  磁极对数
const connectBtn = ref("连接");
const startBtn = ref("启动");
const targetRps = ref(0.00);
const rps_ref = ref(0.0);
const currentRps = ref(0);
const enableIdentify = ref(false);
const enableRsOnline = ref(false);
const enableRsReCalc = ref(false);
const motorIdentified = ref(false);
const motorStarted = ref(false);
const motorState = ref("STOP_IDLE");
const mctrlState = ref("FIRST_RUN");
const errorCode = ref(0);
const baudRates = [
  {
    value: 9600,
    label: '9600',
  },
  {
    value: 115200,
    label: '115200',
  },
]
const baudRate = ref(9600);
const serialPorts = ref(["COM0"]);
const serialPort = ref();

onMounted(() => { //组件挂载时的生命周期执行的方法
  get_avaliable_ports()

  window.setInterval(function timer() {
    if (isConnect.value) {
      get_motor_params()
    }
  }, 1000);

  window.setInterval(function timer() {
    if (isConnect.value) {
      get_motor_status()
    }
  }, 500);

  // window.setInterval(function timer() {
  //   get_avaliable_ports()
  // }, 3000);
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
  if (isConnect.value) {
    isConnect.value = false;
    motorStarted.value = false;
    startBtn.value = "启动";
    connectBtn.value = "连接";
    await cmds.cmd_disconnect_motor(serialPort.value, baudRate.value);
  } else {
    isConnect.value = true;
    connectBtn.value = "断开";
    await cmds.cmd_connect_motor(serialPort.value, baudRate.value);
  }
}

async function get_motor_params() {
  await cmds.cmd_get_motor_params()
    .then((data) => {
      Rs_Ohm.value = data.rs;
      Rs_Ohm_Online.value = data.rs_online;
      Ls_d.value = data.ls_d;
      Ls_q.value = data.ls_q;
      flux.value = data.flux;
      poles.value = data.poles;
      rps_ref.value = data.target_rps;
    })
}

async function get_motor_status() {
  await cmds.cmd_get_motor_status()
    .then((data) => {
      console.log(data);
      currentRps.value = data.rps;
      motorIdentified.value = data.identified;
      errorCode.value = data.error_code;
      motorState.value = data.motor_state;
      mctrlState.value = data.mctrl_state;
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
    await cmds.cmd_update_motor_rps(parseFloat(targetRps.value), poles.value)
  } else {
    motorStarted.value = true;
    startBtn.value = "更新";
    await cmds.cmd_start_motor(parseFloat(targetRps.value), poles.value)
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

</script>

<template>
  <div>
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
                    <el-button @click="connect_motor">{{ connectBtn }}</el-button>
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
                    <el-switch v-model="enableIdentify" :disabled=!isConnect />
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
                    <el-switch v-model="enableRsOnline" :disabled=!isConnect />
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
                    <el-switch v-model="enableRsReCalc" :disabled=!isConnect />
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
                    <el-input v-model="targetRps" :disabled=!isConnect>
                      <template #append>rps</template>
                    </el-input>
                  </el-col>
                  <el-col :span="4">
                    <el-button @click="update_motor_rps" :disabled=!isConnect>{{ startBtn }}</el-button>
                  </el-col>

                  <el-col :span="4" class="ms-1">
                    <el-button @click="stop_motor" :disabled=!motorStarted>停止</el-button>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase title="实时转速" class="mt-2">
          <template #content>
            <VueSpeedometer :height="180" :value="currentRps" :minValue="0" :maxValue="250" :segments="10" />
          </template>
        </cardBase>

      </el-col>

      <el-col :span="12">
        <cardBase title="状态">
          <template #content>
            <el-scrollbar height="453px">
              <el-row :gutter="20">
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
                      <el-input v-model="errorCode" :readonly=true />
                    </el-col>
                    <el-col :span="6">
                      <el-button @click="clear_motor_faults" :disabled=!isConnect>清除错误</el-button>
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>

              <el-row :gutter="20" class="mt-1">
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
                      <label>Rs (Ohm)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="Rs_Ohm" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>
              <el-row class="mt-1">
                <el-col>
                  <el-row :gutter="1">
                    <el-col :span="10">
                      <label style="font-size: 1rem;">Rs Online</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="Rs_Ohm_Online" :readonly=true />
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
                      <label>设定转速 (rps)</label>
                    </el-col>
                    <el-col :span="14">
                      <el-input v-model="rps_ref" :readonly=true />
                    </el-col>
                  </el-row>
                </el-col>
              </el-row>
            </el-scrollbar>
          </template>
        </cardBase>
      </el-col>
    </el-row>
  </div>
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
::v-deep .is-horizontal {
  height: 0px;
  left: 0px;
  display: none;
}

::v-deep .el-scrollbar__wrap {
  overflow-x: hidden;
}
</style>
