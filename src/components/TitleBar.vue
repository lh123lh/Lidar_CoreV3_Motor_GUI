<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, watch } from "vue";
import { appWindow } from "@tauri-apps/api/window";
const max_state_name = ref('window-maximize')
const max_state = ref(false)
watch(max_state, async (newValue) => {
  if (newValue) { //当前状态是最大化
    max_state_name.value = 'window-restore'
    await appWindow.maximize()
  } else {
    max_state_name.value = 'window-maximize'
    await appWindow.unmaximize()
  }
})

async function window_minimize() {
  await appWindow.minimize()
}
function window_maximize() {
  max_state.value = !max_state.value
}
async function window_close() {
  await appWindow.close()
}
</script>

<template>
  <el-row class="titlebar">
    <el-col :span="20" data-tauri-drag-region>
    </el-col>
    <el-col :span="4" class="ms-auto">
      <div id="stage-button">
        <div class="min" viewBox="0 0 1024 1024" name="window-minimize" @click.left="window_minimize">
          <el-icon style="margin-left: 1rem; margin-top: 0.35rem;">
            <SvgIcon iconName="icon-zuixiaohua" />
          </el-icon>
        </div>
        <div class="max" viewBox="0 0 1024 1024" @click.left="window_maximize" :name=max_state_name>
          <el-icon style="margin-left: 1rem; margin-top: 0.35rem;">
            <SvgIcon iconName="icon-zuidahua" />
          </el-icon>
        </div>
        <div class="close" name="multiply" @click.left="window_close" icon-style="monochrome">
          <el-icon style="margin-left: 1rem; margin-top: 0.35rem;">
            <SvgIcon iconName="icon-guanbichuangkou" />
          </el-icon>
        </div>
      </div>
    </el-col>
  </el-row>

</template>

<style scoped>
.titlebar {
  display: flex;
  flex-direction: row;
  height: 30px;
  user-select: none;
  /* background-color: white; */
  top: 0px;
  left: 0;
  right: 0;
}

#stage-button {
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  /*在 Flexbox 中，margin-left: auto; 会将元素推到其容器的末尾，而 margin-right: 0; 在 Flexbox 中不会产生相同的效果。*/
  margin-left: auto;
}

.top {
  width: 30px;
  height: 30px;
}

.top path {
  stroke: red;
  stroke-width: 30;
}

.top:hover {
  cursor: pointer;
}

.top:hover path {
  stroke: #37ff00;
  stroke-width: 30;
}

.min,
.max,
.close {
  /*font-size: 30px;用这个无法设置大小*/
  width: 50px;
  height: 30px;
}

.min path {
  stroke: black;
  width: 24px;
  stroke-width: 1;
}

.max path {
  transform: scale(0.7);
  transform-origin: center;
  stroke: black;
  stroke-width: 20;
}

.min:hover,
.max:hover {
  background-color: #33303020;
  /* border-radius: 5px; */
}

.close:hover {
  fill: white;
  background-color: red;
  /* border-radius: 5px; */
}
</style>
