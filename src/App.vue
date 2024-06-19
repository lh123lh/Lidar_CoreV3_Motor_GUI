<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, watch } from "vue";
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import TitleBar from "./components/TitleBar.vue";
import Motor_main from "./views/Motor_main.vue";

const navs = ref([
  { idx: "1", title: '电机控制', icon: 'icon-control', to: '/' },
  { idx: "2", title: '转速监控', icon: 'icon-monitoring', to: '/speedMonitor' },
  { idx: "3", title: '启停测试', icon: 'icon-stressTest', to: '/stressTest' },
  { idx: "4", title: '设置', icon: 'icon-setting', to: '/setting' },
])
</script>

<template>
  <TitleBar />
  <el-container class="mt-0" style="height: 98vh">
    <el-aside width="180px" class="mt-n3">
      <el-scrollbar>
        <el-menu default-active="1" router style="height: 98vh">
          <img style="width: 30px;" class="ms-3 mb-n1 mt-2" src="../src-tauri/icons/icon.ico" size="small">
          <span style="" class="ms-3 fs-3 fw-bolder">FAST FOC</span>
          </img>

          <el-menu-item v-for="nav in navs" :index=nav.idx :route=nav.to v-wave>
            <el-row :gutter="20">
              <el-col :span="8">
                <el-icon>
                  <SvgIcon :iconName=nav.icon />
                </el-icon>
              </el-col>
              <el-col :span="16" style="text-align: end;">
                <span style="font-weight: 1000; font-size: 1rem;">{{ nav.title }}</span>
              </el-col>
            </el-row>
          </el-menu-item>
        </el-menu>
      </el-scrollbar>
    </el-aside>

    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>

  </el-container>

  <!-- <Motor_main /> -->

</template>

<style scoped></style>
