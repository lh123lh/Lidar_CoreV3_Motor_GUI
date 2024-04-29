<template>
  <div class="ms-1">
    <div ref="chartContainer" style="width: 600px; height: 400px;"></div>
  </div>

</template>

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

let count = 0;

onMounted(() => {
  chartInstance = echarts.init(chartContainer.value);
  const option = {
    title: {
      text: '电机转速'
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        animation: false
      }
    },
    toolbox: {
      feature: {
        dataZoom: {
          yAxisIndex: 'none'
        },
        restore: {},
        // dataView: {},
        saveAsImage: {},
      }
    },
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
        minSpan: 5
      },
      {
        type: 'slider',
        xAxisIndex: 0,
        minSpan: 5,
        bottom: 50
      }
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

    // if (option.xAxis.data.length > 40) {
    //   option.xAxis.data.shift();
    //   option.series[0].data.shift();
    // }

    chartInstance.setOption(option);
  }, 1000);
});
</script>

<style scoped></style>