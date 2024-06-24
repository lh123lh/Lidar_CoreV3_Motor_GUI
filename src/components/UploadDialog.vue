<script setup>
import { ref, watch } from 'vue'
import cmds from '../utils/cmds';
import { open, save } from "@tauri-apps/api/dialog"

const visable = defineModel();
const filePath = ref("");

async function selectUploadFile() {
  await open().then((file) => {
    filePath.value = file;
  })
}

const props = defineProps({
  handleUpload: {
    type: Function,
    required: true
  },

  title: {
    type: String,
    required: true
  },
  uploadBtnName: {
    type: String,
    required: true
  },
  status: {
    type: Boolean,
    required: false,
  }
});

</script>

<template>
  <el-dialog v-model="visable" :title="title" width="500" :lock-scroll="false">
    <span>
      <el-row :gutter="10">
        <el-col :span="19">
          <el-input v-model="filePath" :placeholder="$t('selectFile')" @click="selectUploadFile" :readonly=true
            :disabled="status"></el-input>
        </el-col>
        <el-col :span="4">
          <el-button @click="selectUploadFile" type="success" plain :disabled="status">{{ $t('selectFile') }}</el-button>
        </el-col>
      </el-row>

    </span>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="visable = false" :disabled="status">Cancel</el-button>
        <el-button v-if="!status" type="primary" @click="handleUpload(filePath)">
          {{ uploadBtnName }}
        </el-button>
        <el-button v-else type="primary" loading>
          {{ uploadBtnName }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
