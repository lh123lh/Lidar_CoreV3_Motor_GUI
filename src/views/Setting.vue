<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import PageBase from '../components/PageBase.vue';
import cardBase from '../components/cardBase.vue';
import { useMotorStore } from '../stores/motorState';
import MotorSpecialParamsDialog from '../components/MotorSpecialParamsDialog.vue';
import FwMergeDialog from '../components/FwMergeDialog.vue';

import { appUpdateInfoStore } from '../stores/appUpdateInfo.js'
import { checkUpdate } from '@tauri-apps/api/updater';
import { getVersion } from '@tauri-apps/api/app';

import appUpdateDialog from '../components/appUpdateDialog.vue';

const updateInfo = appUpdateInfoStore();
const updateDialogVisible = ref(false);

const status = useMotorStore();
const motorParamDialog = ref(false);
const fwMergeDialog = ref(false);
const appVersion = ref("");

const { locale } = useI18n()

const language = ref({ value: '中文', lang: 'zh' })
const supportLangs = [
  { value: 'English', lang: 'en' },
  { value: '中文', lang: 'zh' },
]

onMounted(() => {
  getAppVersion();
})

function changeLanguage(val) {
  locale.value = val;
}

async function handleCheckUpdate() {
  try {
    const { shouldUpdate, manifest } = await checkUpdate()

    if (shouldUpdate) {
      updateInfo.updateAvailable = true;
      updateInfo.manifest = manifest.body;
      updateDialogVisible.value = true;
    }
  } catch (error) {
    console.error(error)
  }
}

async function getAppVersion() {
  appVersion.value = await getVersion();
}

</script>

<template>
  <PageBase title="设置">
    <el-row :gutter="5">
      <el-col :xs="12" :sm="12" :md="12" :lg="12" :xl="12">
        <cardBase title="电机设置">
          <template #content>
            <el-row :gutter="5">
              <el-col :span="18" class="setting-item">
                电机特性参数设置
              </el-col>
              <el-col :span="6" style="text-align: end;">
                <el-link :underline="false" :disabled=!status.isConnected class="mb-1" @click="motorParamDialog = true">
                  <el-icon :size="16">
                    <SvgIcon iconName=icon-arrow-right />
                  </el-icon>
                </el-link>
              </el-col>
            </el-row>

            <el-row :gutter="5">
              <el-col :span="18" class="setting-item">
                编码器设置
              </el-col>
              <el-col :span="6" style="text-align: end;">
                <el-link :underline="false" :disabled=!status.isConnected class="mb-1">
                  <el-icon :size="16">
                    <SvgIcon iconName=icon-arrow-right />
                  </el-icon>
                </el-link>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase title="辅助工具" class="mt-1">
          <template #content>
            <el-row :gutter="5" class="setting-item setting-item-clickable" @click="fwMergeDialog = true" v-wave>
              <el-col :span="18">
                合并Hex
              </el-col>
              <el-col :span="6" style="text-align: end;">
                <el-link :underline="false" class="mb-1" @click="fwMergeDialog = true">
                  <el-icon :size="16">
                    <SvgIcon iconName=icon-arrow-right />
                  </el-icon>
                </el-link>
              </el-col>
            </el-row>

          </template>
        </cardBase>
      </el-col>
      <el-col :xs="12" :sm="12" :md="12" :lg="12" :xl="12">
        <cardBase title="系统设置">
          <template #content>
            <el-row :gutter="5">
              <el-col :span="18" class="setting-item">
                语言设置
              </el-col>
              <el-col :span="6" style="text-align: end;">
                <el-select v-model="language" style="width: 100px" @change="changeLanguage(language.lang)">
                  <el-option v-for="item in supportLangs" :key="item.lang" :label="item.value" :value="item"
                    class="fw-bolder" />
                </el-select>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="setting-item setting-item-clickable mt-1" @click="handleCheckUpdate()" v-wave>
              <el-col :span="18">
                检查更新
              </el-col>
              <el-col :span="6" style="text-align: end;">
                <el-link :underline="false" class="mb-1">
                  <el-icon :size="16">
                    <SvgIcon iconName=icon-arrow-right />
                  </el-icon>
                </el-link>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="setting-item mt-1">
              <el-col :span="18">
                应用版本
              </el-col>
              <el-col :span="6" style="text-align: end;">
                v{{ appVersion }}
              </el-col>
            </el-row>

          </template>
        </cardBase>
      </el-col>
    </el-row>


  </PageBase>

  <MotorSpecialParamsDialog v-model="motorParamDialog" />
  <FwMergeDialog v-model="fwMergeDialog" />
  <appUpdateDialog v-model="updateDialogVisible" />
</template>

<style scoped>
.setting-item {
  font-size: 0.9rem;
}

.setting-item-clickable {
  cursor: pointer;
}

.setting-item-clickable:hover {
  background-color: rgb(237, 237, 237);
}
</style>
