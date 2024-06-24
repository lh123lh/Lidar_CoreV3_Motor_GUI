<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useDark, useToggle } from '@vueuse/core';
import PageBase from '../components/PageBase.vue';
import cardBase from '../components/cardBase.vue';
import { useMotorStore } from '../stores/motorState';
import MotorSpecialParamsDialog from '../components/MotorSpecialParamsDialog.vue';
import FwMergeDialog from '../components/FwMergeDialog.vue';
import cmds from '../utils/cmds';

import { ArrowRight, Sunny, Moon, Loading } from '@element-plus/icons-vue'

import { appUpdateInfoStore } from '../stores/appUpdateInfo.js'
import { checkUpdate } from '@tauri-apps/api/updater';
import { getVersion } from '@tauri-apps/api/app';

import appUpdateDialog from '../components/appUpdateDialog.vue';

const updateInfo = appUpdateInfoStore();
const updateDialogVisible = ref(false);
const checkingUpdate = ref(false);

const status = useMotorStore();
const motorParamDialog = ref(false);
const fwMergeDialog = ref(false);
const appVersion = ref("");

const isDark = useDark({ disableTransition: false });
const toggleDark = useToggle(isDark);

const { locale } = useI18n()
const { t } = useI18n()

const language = ref({ value: '中文', lang: 'zh' })
const supportLangs = [
  { value: 'English', lang: 'en' },
  { value: '中文', lang: 'zh' },
]

const darkMode = ref(false);

onMounted(() => {
  getAppVersion();
})

function changeLanguage(val) {
  locale.value = val;
}

async function handleCheckUpdate() {
  checkingUpdate.value = true;
  try {
    const { shouldUpdate, manifest } = await checkUpdate()

    if (shouldUpdate) {
      updateInfo.updateAvailable = true;
      updateInfo.manifest = manifest.body;
      updateDialogVisible.value = true;
    }
    else {
      cmds.notify_success(t('settings.alreadyUpdated'));
    }
  } catch (error) {
    console.error(error);
    cmds.notify_failed(error);
  }
  checkingUpdate.value = false;
}

async function getAppVersion() {
  appVersion.value = await getVersion();
}

const changeTheme = ($eve) => {
  const x = $eve.clientX
  const y = $eve.clientY
  // 计算鼠标点击位置距离视窗的最大圆半径
  const endRadius = Math.hypot(
    Math.max(x, innerWidth - x),
    Math.max(y, innerHeight - y),
  )
  document.documentElement.style.setProperty('--x', x + 'px')
  document.documentElement.style.setProperty('--y', y + 'px')
  document.documentElement.style.setProperty('--r', endRadius + 'px')
  // 判断浏览器是否支持document.startViewTransition
  if (document.startViewTransition) {
    // 如果支持就使用document.startViewTransition方法
    document.startViewTransition(() => {
      toggleDark()
    })
  } else {
    toggleDark()
  }

  darkMode.value = !isDark.value;
}

</script>

<template>
  <PageBase :title="$t('settings.title')">
    <el-row :gutter="5">
      <el-col :xs="12" :sm="12" :md="12" :lg="12" :xl="12">
        <cardBase :title="$t('settings.motorSetting')">
          <template #content>
            <ul>
              <li @click="motorParamDialog = true" :class="{ disabled: !status.isConnected }">
                <div class="setting-item setting-item-clickable" v-wave>
                  <div>
                    {{ $t('settings.motorSpecialParamsSetting') }}
                  </div>
                  <el-link :underline="false" class="ms-auto">
                    <el-icon :size="20">
                      <ArrowRight />
                    </el-icon>
                  </el-link>
                </div>
              </li>

              <li class="disabled">
                <div class="setting-item setting-item-clickable" v-wave>
                  <div>
                    {{ $t('settings.encoderSetting') }}
                  </div>
                  <el-link :underline="false" :disabled=!status.isConnected class="ms-auto">
                    <el-icon :size="20">
                      <ArrowRight />
                    </el-icon>
                  </el-link>
                </div>
              </li>
            </ul>
          </template>
        </cardBase>

        <cardBase :title="$t('settings.tools')" class="mt-1">
          <template #content>
            <ul>
              <li @click="fwMergeDialog = true">
                <div class="setting-item setting-item-clickable" v-wave>
                  <div>
                    {{ $t('settings.mergeHex') }}
                  </div>
                  <el-link :underline="false" class="ms-auto" @click="fwMergeDialog = true">
                    <el-icon :size="20">
                      <ArrowRight />
                    </el-icon>
                  </el-link>
                </div>
              </li>
            </ul>

          </template>
        </cardBase>
      </el-col>
      <el-col :xs="12" :sm="12" :md="12" :lg="12" :xl="12">
        <cardBase :title="$t('settings.systemSetting')">
          <template #content>
            <ul>
              <li>
                <div class="setting-item">
                  <div>
                    {{ $t('settings.language') }}
                  </div>
                  <el-select v-model="language" style="width: 100px" @change="changeLanguage(language.lang)"
                    class="ms-auto">
                    <el-option v-for="item in supportLangs" :key="item.lang" :label="item.value" :value="item"
                      class="fw-bolder" />
                  </el-select>
                </div>
              </li>

              <li>
                <div class="setting-item">
                  <div>
                    {{ $t('settings.darkMode') }}
                  </div>
                  <el-switch class="ms-auto" v-model="darkMode" :active-action-icon="Moon"
                    :inactive-action-icon="Sunny" @click="changeTheme" />
                </div>
              </li>

              <li @click="handleCheckUpdate()">
                <div class="setting-item setting-item-clickable" v-wave>
                  <div>
                    {{ $t('settings.checkUpdate') }}
                  </div>
                  <el-link :underline="false" class="ms-auto">
                    <el-icon v-if="!checkingUpdate" :size="20">
                      <ArrowRight />
                    </el-icon>
                    <el-icon v-else :size="20" class="is-loading">
                      <Loading />
                    </el-icon>
                  </el-link>
                </div>
              </li>

              <li>
                <div class="setting-item">
                  <div>
                    {{ $t('settings.appVersion') }}
                  </div>
                  <div class="ms-auto">
                    v{{ appVersion }}
                  </div>
                </div>
              </li>
            </ul>

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
.disabled {
  pointer-events: none;
  opacity: 0.6;
}

.setting-item {
  -webkit-tap-highlight-color: transparent;
  background-color: transparent;
  outline: 0px;
  border: 0px;
  margin: 0px;
  margin-left: -10px;
  margin-right: -10px;
  border-radius: 0px;
  user-select: none;
  vertical-align: middle;
  appearance: none;
  color: inherit;
  display: flex;
  -webkit-box-flex: 1;
  flex-grow: 1;
  -webkit-box-pack: start;
  justify-content: flex-start;
  -webkit-box-align: center;
  align-items: center;
  position: relative;
  text-decoration: none;
  min-width: 0px;
  box-sizing: border-box;
  text-align: left;
  padding: 8px 8px;
  transition: background-color 150ms;
}

.setting-item-clickable {
  cursor: pointer;
}

.setting-item-clickable:hover {
  background-color: rgb(237, 237, 237);
  color: black;
}
</style>

<style>
::view-transition-old(*) {
  animation: none;
}

::view-transition-new(*) {
  animation: clip .5s ease-in;
}

::view-transition-old(root) {
  z-index: 1;
}

::view-transition-new(root) {
  z-index: 9999;
}

html.dark::view-transition-old(*) {
  animation: clip .5s ease-in reverse;
}

html.dark::view-transition-new(*) {
  animation: none;
}

html.dark::view-transition-old(root) {
  z-index: 9999;
}

html.dark::view-transition-new(root) {
  z-index: 1;
}

@keyframes clip {
  from {
    clip-path: circle(0% at var(--x) var(--y));
  }

  to {
    clip-path: circle(var(--r) at var(--x) var(--y));
  }
}
</style>
