<script setup>
import { onMounted, ref } from 'vue';
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

let count = 0;

onMounted(() => {
  chartInstance = echarts.init(chartContainer.value);
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
      boundaryGap: [0, '100%']
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
        data: []
      }
    ]
  };
  chartInstance.setOption(option);

  setInterval(() => {
    count += 1;
    const speed = 20.0 + (Math.random() * 0.4 - 0.5); // 模拟转速数据

    option.xAxis.data.push(count);
    option.series[0].data.push(speed);

    // console.log(trendPoints.value, option.xAxis.data.length)

    // if (trendPoints.value > 0) {
    //   if (option.xAxis.data.length > trendPoints.value) {
    //     option.xAxis.data.shift();
    //     option.series[0].data.shift();
    //   }
    // }

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
    recording.value = false;
  } else {
    recording.value = true;
    dialogVisable.value = false;
  }
}

</script>

<template>
  <div class="ms-1">
    <el-row :gutter="5">
      <el-col :xs="16" :sm="16" :md="16" :lg="16" :xl="16">
        <cardBase title="转速趋势">
          <template #content>
            <div ref="chartContainer" style="width: 600px; height: 460px;"></div>
          </template>
        </cardBase>
      </el-col>
      <el-col :xs="8" :sm="8" :md="8" :lg="8" :xl="8">
        <!-- <cardBase title="趋势表设置">
          <template #content>
            <el-row :gutter="5" class="mt-n3">
              <el-col :span="8">
                <label>观测点数</label>
              </el-col>
              <el-col :span="16">
                <el-input v-model="trendPoints">
                </el-input>
              </el-col>
            </el-row>
          </template>
        </cardBase> -->

        <cardBase title="转速记录" class="mt-0">
          <template #content>
            <el-row :gutter="5" class="mt-n3">
              <el-col :span="8">
                <label>记录位置:</label>
              </el-col>
              <el-col :span="16">
                <el-input v-model="recodePath" @click="handleRecodeRps">
                </el-input>
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-col :span="8">
                <label>记录时长:</label>
              </el-col>
              <el-col :span="16">
                00:01:30
              </el-col>
            </el-row>

            <el-row :gutter="5" class="mt-1">
              <el-button type="primary" v-if="!recording" @click="dialogVisable = true" plain
                class="ms-auto">开始记录</el-button>
              <el-button type="danger" v-else @click="startRecodeRps" plain class="ms-auto">停止记录</el-button>
            </el-row>

          </template>
        </cardBase>
      </el-col>
    </el-row>
  </div>

  <el-dialog v-model="dialogVisable" title="记录转速" width="500">
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

<style scoped></style>