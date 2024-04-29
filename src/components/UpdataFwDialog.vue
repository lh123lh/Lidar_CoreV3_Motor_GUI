<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open, save } from "@tauri-apps/api/dialog"

const visable = defineModel();
const startUpdate = ref(false);
const fwPath = ref("");

async function update_fw() {
}

async function handleFwUpload() {
  await open().then((file) => {
    console.log(file);
    fwPath.value = file;
  })
}

</script>

<template>
  <el-dialog v-model="visable" title="固件升级" width="500">
    <span>
      <el-row :gutter="10">
        <el-col :span="19">
          <el-input v-model="fwPath" placeholder="固件" size="normal" @click="handleFwUpload" :readonly=true></el-input>
        </el-col>
        <el-col :span="4">
          <el-button @click="handleFwUpload" type="success" plain>选择固件</el-button>
        </el-col>
      </el-row>
      
      <!-- <el-progress :text-inside="true" :stroke-width="26" :percentage="70" /> -->

    </span>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="visable = false">Cancel</el-button>
        <el-button type="primary" @click="update_fw">
          升级
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
