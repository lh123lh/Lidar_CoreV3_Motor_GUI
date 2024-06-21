<script setup>
import { ref, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event';
import { appUpdateInfoStore } from '../stores/appUpdateInfo.js'
import { installUpdate } from '@tauri-apps/api/updater'
import MarkdownDisplay from '../components/MarkdownDisplay.vue';

const visable = defineModel();
const status = ref(false);
const updateInfo = appUpdateInfoStore();
const progress = ref(100.0);

onMounted(() => {
  listen("tauri://update-download-progress", event => {
    console.log(event.payload);
    progress.value = parseInt(event.payload.chunkLength) / parseInt(event.payload.contentLength);
  });
})

async function handleUpdate() {
  status.value = true;
  // Install the update. This will also restart the app on Windows!
  await installUpdate()

  // On macOS and Linux you will need to restart the app manually.
  // You could use this step to display another confirmation dialog.
  await relaunch()
}

</script>

<template>
  <el-dialog v-model="visable" title="版本更新" width="500" :lock-scroll="false">
    <div class="ms-5">
      <MarkdownDisplay :markdown="updateInfo.manifest" />
    </div>
    <el-progress :percentage="progress" :stroke-width="5" striped striped-flow :duration="50" :show-text="false"
      class="mt-2" />

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="visable = false" :disabled="status">Cancel</el-button>
        <el-button v-if="!status" type="primary" @click="handleUpdate()">
          下载
        </el-button>
        <el-button v-else type="primary" loading>
          下载中
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
