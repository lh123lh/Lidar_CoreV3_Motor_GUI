<script setup>
import { onMounted, ref, onUpdated, computed } from 'vue';
// Import only necessary parts of ECharts
import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  ToolboxComponent,
  DataZoomComponent,
  GridComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import { open, save } from "@tauri-apps/api/dialog"
import cardBase from '../components/cardBase.vue';
import PageBase from '../components/PageBase.vue';
import cmds from '../utils/cmds';
import { updater } from '@tauri-apps/api';
import { useMotorStore } from '../stores/motorState'

const store = useMotorStore()

// Register the required components
echarts.use([
  TitleComponent,
  TooltipComponent,
  ToolboxComponent,
  DataZoomComponent,
  LineChart,
  CanvasRenderer,
  GridComponent
]);

const chartContainer = ref(null);
let chartInstance = null;

const trendPoints = ref(0);
const recodePath = ref("");
const recording = ref(false);
const dialogVisable = ref(false);
const startObserv = ref(false);
const recordTimerFormated = ref();

let count = 0;
let recordTimer = 0;

const option = {
  // title: {
  //   text: '电机转速'
  // },
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      animation: false
    }
  },
  // toolbox: {
  //   feature: {
  //     dataZoom: {
  //       yAxisIndex: 'none'
  //     },
  //     restore: {},
  //     // dataView: {},
  //     saveAsImage: {},
  //   }
  // },
  xAxis: {
    type: 'category',
    data: []
  },
  yAxis: {
    type: 'value',
    boundaryGap: [0, '100%'],
  },
  dataZoom: [
    {
      type: 'inside',
      xAxisIndex: 0,
      minSpan: 2,
      zoomOnMouseWheel: "shift",
    },
    {
      type: 'inside',
      yAxisIndex: 0,
      minSpan: 1,
      zoomOnMouseWheel: "ctrl",
    },
    // {
    //   type: 'slider',
    //   xAxisIndex: 0,
    //   minSpan: 5,
    //   bottom: 50
    // }
  ],
  series: [
    {
      name: '转速',
      type: 'line',
      showSymbol: false,
      data: [],
    }
  ],
  //调整此处大小即可控制空白
  grid: { x: 40, y: 10, x2: 30, y2: 20 },
};

onUpdated(() => {
  chartInstance.resize();
})

onMounted(() => {
  chartInstance = echarts.init(chartContainer.value);
  chartInstance.setOption(option);
  window.addEventListener("resize", () => {
    chartInstance.resize();
  });

  setInterval(() => {
    if (startObserv.value) {
      count += 1;
      // const speed = (20.0 + (Math.random() * 0.4 - 0.5)).toFixed(2); // 模拟转速数据

      option.xAxis.data.push(count);
      option.series[0].data.push(store.currRps);

      if (trendPoints.value > 0) {
        if (option.xAxis.data.length > trendPoints.value) {
          option.xAxis.data.shift();
          option.series[0].data.shift();
        }
      }
    }

    if (recording.value) {
      recordTimer += 1;
      recordTimerFormated.value = cmds.formatSeconds(recordTimer);
    }

    chartInstance.setOption(option);
  }, 1000);
});

async function handleRecodeRps() {
  await save({
    filters: [{
      name: 'File',
      extensions: ['csv']
    }]
  }).then((file) => {
    recodePath.value = file;
  })
}

async function startRecodeRps() {
  if (recording.value) {
    await cmds.cmd_stop_record_rps()
      .then((data) => {
        recording.value = false;
      })
  } else {
    await cmds.cmd_start_record_rps(recodePath.value)
      .then((data) => {
        recordTimer = 0;
        recording.value = true;
        dialogVisable.value = false;
      })
  }
}

async function handleStartObserv() {
  if (startObserv.value) {
    startObserv.value = false;
  } else {
    count = 0;
    option.xAxis.data = [];
    option.series[0].data = [];
    startObserv.value = true;
  }
}

function onResize() {

}

</script>

<template>
  <PageBase title="转速监控">
    <el-row :gutter="5">
      <el-col :span="16">
        <cardBase title="转速趋势">
          <template #content>
            <div ref="chartContainer" class="monitor"></div>
          </template>
        </cardBase>
      </el-col>
      <el-col :span="8">
        <cardBase title="趋势表设置">
          <template #content>
            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>观测点数:</label>
              </el-col>
              <el-col :span="16">
                <el-input v-model="trendPoints" :disabled="startObserv">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="24" style="text-align: end;">
                <el-button type="primary" @click="handleStartObserv" v-if="!startObserv" plain
                  class="ms-auto">开始观测</el-button>
                <el-button type="danger" @click="handleStartObserv" v-else plain class="ms-auto"
                  :disabled="recording">停止观测</el-button>
              </el-col>
            </el-row>
          </template>
        </cardBase>

        <cardBase title="转速记录" class="mt-1">
          <template #content>
            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>记录位置:</label>
              </el-col>
              <el-col :span="16">
                <el-input v-model="recodePath" @click="handleRecodeRps" :disabled="!startObserv || recording">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>记录时长:</label>
              </el-col>
              <el-col :span="16">
                {{ recordTimerFormated }}
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="24" style="text-align: end;">
                <el-button type="primary" v-if="!recording" @click="dialogVisable = true" plain class="ms-auto"
                  :disabled="!startObserv">开始记录</el-button>
                <el-button type="danger" v-else @click="startRecodeRps" plain class="ms-auto">停止记录</el-button>
              </el-col>
            </el-row>

          </template>
        </cardBase>
      </el-col>
    </el-row>
  </PageBase>

  <el-dialog v-model="dialogVisable" title="提示" width="500">
    <span>
      注意: 文件默认保存为CSV格式!
    </span>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogVisable = false">取消</el-button>
        <el-button type="primary" @click="startRecodeRps">
          确定
        </el-button>
      </div>
    </template>
  </el-dialog>

</template>

<style scoped>
.monitor {
  width: 100%; 
  height: 79vh;
}

@media (min-width: 1200px) {
  .monitor {
      height: 84.5vh;
  }
}
</style>