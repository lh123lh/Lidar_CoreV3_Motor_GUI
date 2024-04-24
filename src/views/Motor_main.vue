<script setup>
import { ref, onMounted, nextTick, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import cmds from '../api/cmds';
import cardBase from "../components/cardBase.vue";
import VueSpeedometer from "vue-speedometer"

const isConnect = ref(false);
const Rs_Ohm = ref(0.001);
const Rs_Ohm_Online = ref(0.001);
const Ls_d = ref(0.001);
const Ls_q = ref(0.001);
const flux = ref(100);
const connectBtn = ref("连接");
const startBtn = ref("启动");
const targetRps = ref(0);
const currentRps = ref(15.5);
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

  // window.setInterval(function timer() {
  //   get_avaliable_ports()
  // }, 3000);
})

async function connect_motor() {
  if (isConnect.value) {
    isConnect.value = false;
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
    })
}

async function get_avaliable_ports() {
  await cmds.cmd_get_avaliable_ports()
    .then((data) => {
      serialPorts.value = data;
    })
}

</script>

<template>
  <div>
    <el-row :gutter="5">
      <el-col :span="12">
        <cardBase title="电机配置">
          <template #content>
            <el-row :gutter="5">
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

            <el-row :gutter="5" class="mt-2">
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

            <el-row :gutter="5" class="mt-2">
              <el-col>
                <el-row :gutter="5">
                  <el-col :span="5">
                    <label>目标转速</label>
                  </el-col>
                  <el-col :span="10">
                    <el-input v-model="targetRps" style="font-size: 0.8rem" :disabled=!isConnect>
                      <template #append>rps</template>
                    </el-input>
                  </el-col>
                  <el-col :span="4">
                    <el-button :disabled=!isConnect>{{ startBtn }}</el-button>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase title="电机状态" class="mt-2">
          <template #content>
            <VueSpeedometer :height="180" :value="currentRps" :minValue="0" maxValue="200" :segments="10" />
          </template>

        </cardBase>
      </el-col>
      <el-col :span="12">
        <cardBase title="电机参数">
          <template #content>
            <el-row :gutter="20">
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
            <el-row class="mt-2">
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

            <el-row :gutter="20" class="mt-2">
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
            <el-row class="mt-2">
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

            <el-row :gutter="20" class="mt-2">
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

          </template>
        </cardBase>
      </el-col>
    </el-row>
  </div>
</template>
