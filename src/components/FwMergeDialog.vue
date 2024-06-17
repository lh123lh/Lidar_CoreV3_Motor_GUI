<script setup>
import { ref, watch } from 'vue'
import cmds from '../utils/cmds';
import { open, save } from "@tauri-apps/api/dialog"

const visable = defineModel();
const bootPath = ref("");
const appPath = ref("");
const mergedPath = ref("");

async function selectBootHexFile() {
  await open({
    filters: [{
      name: 'Boot',
      extensions: ['hex']
    }]
  }).then((file) => {
    bootPath.value = file;
  })
}

async function selectAppHexFile() {
  await open({
    filters: [{
      name: 'App',
      extensions: ['hex']
    }]
  }).then((file) => {
    appPath.value = file;
  })
}

async function selectMergedFile() {
  await save({
    filters: [{
      name: 'File',
      extensions: ['hex']
    }]
  }).then((file) => {
    mergedPath.value = file;
  })
}

async function hanleMergeFile() {
  await cmds.cmd_merge_motor_fw(bootPath.value, appPath.value, mergedPath.value)
    .then(() => {
      cmds.notify_success("合并完成");
    })
    .catch(() => {
      cmds.notify_failed("合并失败");
    })
}

</script>

<template>
  <el-dialog v-model="visable" title="固件合并" width="500">
    <span>
      <el-row :gutter="10">
        <el-col :span="4">
          Boot Hex
        </el-col>
        <el-col :span="15">
          <el-input v-model="bootPath" placeholder="选择Boot Hex" size="normal" @click="selectBootHexFile" :readonly=true
            :disabled="status"></el-input>
        </el-col>
        <el-col :span="4">
          <el-button @click="selectBootHexFile" type="success" plain :disabled="status">浏览</el-button>
        </el-col>
      </el-row>

      <el-row :gutter="10" class="mt-1">
        <el-col :span="4">
          App Hex
        </el-col>
        <el-col :span="15">
          <el-input v-model="appPath" placeholder="选择App Hex" size="normal" @click="selectAppHexFile" :readonly=true
            :disabled="status"></el-input>
        </el-col>
        <el-col :span="4">
          <el-button @click="selectAppHexFile" type="success" plain :disabled="status">浏览</el-button>
        </el-col>
      </el-row>

      <el-row :gutter="10" class="mt-1">
        <el-col :span="4">
          目标 Hex
        </el-col>
        <el-col :span="15">
          <el-input v-model="mergedPath" placeholder="选择目标 Hex" size="normal" @click="selectMergedFile" :readonly=true
            :disabled="status"></el-input>
        </el-col>
        <el-col :span="4">
          <el-button @click="selectMergedFile" type="success" plain :disabled="status">浏览</el-button>
        </el-col>
      </el-row>

    </span>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="visable = false" :disabled="status">Cancel</el-button>
        <el-button type="primary" @click="hanleMergeFile()">
          合并
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
