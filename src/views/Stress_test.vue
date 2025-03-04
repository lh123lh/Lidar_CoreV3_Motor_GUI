<script setup>
import { onMounted, ref, onUpdated, computed } from 'vue';
import { open, save } from "@tauri-apps/api/dialog"
import cardBase from '../components/cardBase.vue';
import PageBase from '../components/PageBase.vue';
import cmds from '../utils/cmds';
import { updater } from '@tauri-apps/api';
import { useMotorStore, useRelayStore } from '../stores/motorState'
import { listen } from '@tauri-apps/api/event';

const store = useMotorStore()
const relayStore = useRelayStore();

const totalCnt = ref(1000);
const targetRps = ref(2);
const coldDuration = ref(10);
const rotateDuration = ref(20);
const successCnt = ref(0);
const failedCnt = ref(0);
const progress = ref(0.0);
const testDuration = ref(0);
const testDurationFormated = ref("00:00:00");
const logs = ref([]);

const serialPorts = ref(["COM0"]);
const serialPort = ref();

const test_param = ref({
  target_rps: 0.0,
  total_count: 1000,
  rotate_duration: 20,
  cold_duration: 20,
  has_relay: false,
});

onMounted(() => {
  get_avaliable_ports();

  setInterval(() => {
    if (store.isTesting) {
      testDuration.value += 1;
      testDurationFormated.value = cmds.formatSeconds(testDuration.value);
      get_startup_test_result();
    }
  }, 1000);

  window.setInterval(function timer() {
    get_avaliable_ports()
  }, 3000);

  listen('log_event', event => {
    // logs.value.push(event.payload);

    const { message, level, timestamp } = event.payload;
    logs.value.push({ id: logs.value.length, message, level, timestamp });
  });

});

async function handleStartTest() {
  if (!store.isTesting) {
    test_param.value.target_rps = parseFloat(targetRps.value);
    test_param.value.total_count = parseInt(totalCnt.value);
    test_param.value.rotate_duration = parseInt(rotateDuration.value);
    test_param.value.cold_duration = parseInt(coldDuration.value);
    test_param.value.has_relay = relayStore.isConnected;

    await cmds.cmd_start_startup_test(test_param.value)
      .then((data) => {
        store.isTesting = true;
        testDuration.value = 0;
        logs.value = [];
      })
  } else {
    await cmds.cmd_stop_startup_test()
      .then((data) => {
        store.isTesting = false;
      })
  }
}

async function get_startup_test_result() {
  if (store.isTesting) {
    await cmds.cmd_get_startup_test_result()
      .then((data) => {
        successCnt.value = data.success_cnt;
        failedCnt.value = data.failed_cnt;
        progress.value = data.progress;

        if (successCnt.value + failedCnt.value >= totalCnt.value) {
          store.isTesting = false;
        }
      })
  }
}

const tableRowClassName = ({ row }) => {
  // console.log(row);
  if (row.level === 'danger') {
    return 'danger-row';
  } else if (row.level === 'warning') {
    return 'warning-row';
  }

  return '';
};

async function get_avaliable_ports() {
  await cmds.cmd_get_avaliable_ports()
    .then((data) => {
      serialPorts.value = data;
    })
}

async function handlerelayConnect() {
  if (!relayStore.isConnected) {
    await cmds.cmd_connect_relay(serialPort.value, 9600)
      .then((data) => {
        relayStore.isConnected = true;
      })
  } else {
    await cmds.cmd_disconnect_relay()
      .then((data) => {
        relayStore.isPowerOn = false;
        relayStore.isConnected = false;
      })
  }
}

async function handleRelayPowerSet() {
  if (!relayStore.isPowerOn) {
    await cmds.cmd_set_relay_power(true)
      .then((data) => {
        relayStore.isPowerOn = true;
      })
  } else {
    await cmds.cmd_set_relay_power(false)
      .then((data) => {
        relayStore.isPowerOn = false;
      })
  }
}

</script>

<template>
  <PageBase :title="$t('startStop.title')">
    <el-row :gutter="5">
      <el-col :span="16">
        <cardBase :title="$t('startStop.logs')">
          <template #content>
            <div class="logs">
              <el-scrollbar max-height="100%" class="mt-1">
                <el-table :data="logs" :row-class-name="tableRowClassName" class="mt-n2">
                  <el-table-column width="250" prop="timestamp" :label="$t('startStop.timeStamp')"></el-table-column>
                  <el-table-column prop="message" :label="$t('startStop.message')"></el-table-column>
                </el-table>
              </el-scrollbar>
            </div>
          </template>
        </cardBase>
      </el-col>
      <el-col :span="8">
        <cardBase :title="$t('startStop.relay')" class="mt-0">
          <template #content>
            <el-row :gutter="5" class="mt-1">
              <el-col :span="12">
                <label>{{ $t('startStop.relayPort') }}:</label>
              </el-col>
              <el-col :span="12">
                <el-select v-model="serialPort" placeholder="Serial Port" :disabled=relayStore.isConnected>
                  <el-option v-for="item in serialPorts" :value="item" />
                </el-select>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="24" style="text-align: end;">
                <el-button type="primary" @click="handlerelayConnect" v-if="!relayStore.isConnected" plain
                  class="me-2">{{ $t('startStop.relayConnect') }}</el-button>
                <el-button type="primary" @click="handlerelayConnect" v-else plain class="me-2">{{
                  $t('startStop.relayDisconnect') }}</el-button>

                <el-button type="primary" @click="handleRelayPowerSet" v-if="!relayStore.isPowerOn" plain
                  class="ms-auto" :disabled=!relayStore.isConnected>{{ $t('startStop.relayStart') }}</el-button>
                <el-button type="danger" @click="handleRelayPowerSet" v-else plain class="ms-auto">{{ $t('stop')
                  }}</el-button>
              </el-col>
            </el-row>

          </template>
        </cardBase>

        <cardBase :title="$t('startStop.settings')" class="mt-1">
          <template #content>
            <el-row :gutter="5" class="mt-1">
              <el-col :span="12">
                <label>{{ $t('startStop.testCnt') }}:</label>
              </el-col>
              <el-col :span="12">
                <el-input v-model="totalCnt" :disabled="store.isTesting">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="12">
                <label>{{ $t('startStop.targetRps') }}(rps):</label>
              </el-col>
              <el-col :span="12">
                <el-input v-model="targetRps" :disabled="store.isTesting">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="12">
                <label>{{ $t('startStop.rotateDuration') }}(s):</label>
              </el-col>
              <el-col :span="12">
                <el-input v-model="rotateDuration" :disabled="store.isTesting">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="12">
                <label>{{ $t('startStop.coldDuration') }}(s):</label>
              </el-col>
              <el-col :span="12">
                <el-input v-model="coldDuration" :disabled="store.isTesting">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="24" style="text-align: end;">
                <el-button type="primary" @click="handleStartTest" v-if="!store.isTesting" plain class="ms-auto"
                  :disabled=!store.isConnected>{{ $t('start') }}</el-button>
                <el-button type="danger" @click="handleStartTest" v-else plain class="ms-auto">{{ $t('stop')
                  }}</el-button>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase :title="$t('startStop.status')" class="mt-1">
          <template #content>
            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>{{ $t('startStop.SuccessCnt') }}:</label>
              </el-col>
              <el-col :span="16">
                {{ successCnt }}
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>{{ $t('startStop.FailedCnt') }}:</label>
              </el-col>
              <el-col :span="16">
                {{ failedCnt }}
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>{{ $t('startStop.testDuration') }}:</label>
              </el-col>
              <el-col :span="16">
                {{ testDurationFormated }}
              </el-col>
            </el-row>

          </template>
        </cardBase>

      </el-col>
    </el-row>
  </PageBase>
</template>

<style>
.el-table .danger-row {
  --el-table-tr-bg-color: var(--el-color-danger-light-5);
}

.el-table .warning-row {
  --el-table-tr-bg-color: var(--el-color-warning-light-5);
}

.el-table .success-row {
  --el-table-tr-bg-color: var(--el-color-success-light-5);
}

.logs {
  width: 100%;
  height: 80.5vh;
}

@media (min-width: 1200px) {
  .logs {
    height: 85.5vh;
  }
}
</style>