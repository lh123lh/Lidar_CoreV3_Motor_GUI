<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted } from "vue";
import TitleBar from "./components/TitleBar.vue";
import { appUpdateInfoStore } from './stores/appUpdateInfo.js'
import {
  checkUpdate,
} from '@tauri-apps/api/updater'

import { useI18n } from 'vue-i18n';

import appUpdateDialog from './components/appUpdateDialog.vue';

const { t } = useI18n();

const updateInfo = appUpdateInfoStore();
const updateDialogVisible = ref(false);

const navs = ref([
  { idx: "1", title: t('menu.motorCtrl'), icon: 'icon-control', to: '/' },
  { idx: "2", title: t('menu.spdMonitor'), icon: 'icon-monitoring', to: '/speedMonitor' },
  { idx: "3", title: t('menu.startupTest'), icon: 'icon-stressTest', to: '/stressTest' },
  { idx: "4", title: t('menu.setting'), icon: 'icon-setting', to: '/setting' },
])

onMounted(() => {
  checkAppUpdate();
})

async function checkAppUpdate() {
  try {
    const { shouldUpdate, manifest } = await checkUpdate()

    if (shouldUpdate) {
      updateInfo.updateAvailable = true;
      updateInfo.manifest = manifest.body;
    }
  } catch (error) {
    console.error(error)
  }
}

</script>

<template>
  <el-container>
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu default-active="1" router style="height: 100vh;">

          <div style=" padding-top: 1.8rem;" class="logo-title" data-tauri-drag-region>
            <img style="width: 2rem;" src="./assets/motor.png" />
            <el-badge v-if="updateInfo.updateAvailable" value="new" class="updateBtn" :offset="[-15, -10]"
              @click="updateDialogVisible = true">
              <span class="ms-3 fs-1 fw-bolder">FAST FOC</span>
            </el-badge>
            <span v-else class="ms-3 fs-1 fw-bolder">FAST FOC</span>

          </div>

          <el-menu-item v-for="nav in navs" :index=nav.idx :route=nav.to v-wave>
            <el-row :gutter="5">
              <el-col :span="10">
                <el-icon>
                  <SvgIcon :iconName=nav.icon />
                </el-icon>
              </el-col>
              <el-col :span="14">
                <span style="font-weight: 1000; font-size: 1rem;">{{ nav.title }}</span>
              </el-col>
            </el-row>
          </el-menu-item>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-container>
      <el-header>
        <TitleBar />
      </el-header>

      <el-main>
        <router-view v-slot="{ Component }">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </router-view>
      </el-main>

    </el-container>
  </el-container>

  <appUpdateDialog v-model="updateDialogVisible" />

</template>

<style scoped>
.logo-title {
  display: flex;
  align-items: center;
  justify-content: center;
}

.el-header {
  --el-header-padding: 0px;
  --el-header-height: 30px;
}

.el-main {
  --el-main-padding: 0px;
}

.el-menu-item {
  display: block;
}

.updateBtn {
  cursor: pointer;
}
</style>
