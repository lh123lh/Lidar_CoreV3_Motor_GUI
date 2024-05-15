<script setup>
import { onMounted, ref, onUpdated, computed } from 'vue';
import { open, save } from "@tauri-apps/api/dialog"
import cardBase from '../components/cardBase.vue';
import PageBase from '../components/PageBase.vue';
import cmds from '../api/cmds';
import { updater } from '@tauri-apps/api';
import { useMotorStore } from '../stores/motorState'

const store = useMotorStore()

const totalCnt = ref(1000);
const ratateDuration = ref(10);
const successCnt = ref(0);
const failedCnt = ref(0);
const testDuration = ref(0);
const testDurationFormated = ref("00:00:00");
const started = ref(false);

// onUpdated(() => {
//   chartInstance.resize();
// })

onMounted(() => {
  setInterval(() => {
    if (started.value) {
      testDuration.value += 1;
      testDurationFormated.value = cmds.formatSeconds(testDuration.value);
    }
  }, 1000);
});

function handleStartTest() {
  if (!started.value) {
    started.value = true;
  } else {
    started.value = false;
  }
}

</script>

<template>
  <PageBase title="启停测试">
    <el-row :gutter="5">
      <el-col :span="16">
        <cardBase title="日志">
          <template #content>
            <div style="width: 100%; height: 79vh;">
              <el-scrollbar max-height="29.0rem" class="mt-n3">

              </el-scrollbar>
            </div>
          </template>
        </cardBase>
      </el-col>
      <el-col :span="8">
        <cardBase title="设置">
          <template #content>
            <el-row :gutter="5" class="mt-n3">
              <el-col :span="12">
                <label>测试次数:</label>
              </el-col>
              <el-col :span="12">
                <el-input v-model="totalCnt" :disabled="started">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="12">
                <label>单次测试时长:</label>
              </el-col>
              <el-col :span="12">
                <el-input v-model="ratateDuration" :disabled="started">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-button type="primary" @click="handleStartTest" v-if="!started" plain class="ms-auto">开始测试</el-button>
              <el-button type="danger" @click="handleStartTest" v-else plain class="ms-auto">停止测试</el-button>
            </el-row>
          </template>
        </cardBase>

        <cardBase title="状态" class="mt-1">
          <template #content>
            <el-row :gutter="5" class="mt-n3">
              <el-col :span="8">
                <label>成功次数:</label>
              </el-col>
              <el-col :span="16">
                {{ successCnt }}
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>失败次数:</label>
              </el-col>
              <el-col :span="16">
                {{ failedCnt }}
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>测试时长:</label>
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