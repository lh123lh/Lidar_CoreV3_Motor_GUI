<script setup>
import { onMounted, ref, onUpdated, computed, watch } from 'vue';
// Import only necessary parts of ECharts
import * as echarts from 'echarts/core';
import { GaugeChart } from 'echarts/charts'
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
  CanvasRenderer,
  GridComponent,
  GaugeChart
]);

const chartContainer = ref(null);
let chartInstance = null;

const option = {
  textStyle: {
    fontSize: 6
  },
  series: [
    {
      type: 'gauge',
      min: 0,
      max: 360,
      radius: '100%',
      startAngle: 0,  // 起始角度
      endAngle: 360,  // 结束角度
      center: ['50%', '50%'],
      splitNumber: 12,
      axisLine: {
        lineStyle: {
          width: 10,
          color: [
            [0.25, '#66CDAA'],
            [0.5, '#6495ED'],
            [0.75, '#37a2da'],
            [1, '#fd666d']
          ]
        }
      },
      pointer: {
        itemStyle: {
          color: 'auto'
        }
      },
      axisTick: {
        distance: -20,
        length: 8,
        lineStyle: {
          color: '#fff',
          width: 2
        }
      },
      splitLine: {
        distance: -30,
        length: 30,
        lineStyle: {
          color: '#fff',
          width: 2
        }
      },
      axisLabel: {
        color: 'inherit',
        distance: 12,
        fontSize: 10,
        formatter: (value) => {
          return value === 360 ? 0 : value;
        }
      },
      detail: {
        valueAnimation: false,
        formatter: '{value} °',
        color: 'inherit',
        fontSize: 15,
        offsetCenter: [0, '60%']  // 调整详情文字位置
      },
      data: [
        {
          value: 0,
        }
      ],
    }
  ],

}

const props = defineProps({
  value: {
    type: Number,
    required: true
  },
  width: {
    type: String,
    default: '100%' // 默认宽度
  },
  height: {
    type: String,
    default: '30vh' // 默认高度
  },
});

onMounted(() => {
  chartInstance = echarts.init(chartContainer.value);
  chartInstance.setOption(option);
  window.addEventListener("resize", () => {
    chartInstance.resize();
  });
});

watch(() => props.value, (newSpeed) => {
  if (chartInstance) {
    chartInstance.setOption({
      series: [{
        data: [{ value: newSpeed }]
      }]
    });
  }
});

</script>

<template>
  <div ref="chartContainer" :style="{ width: props.width, height: props.height, textAlign: 'center' }"></div>
</template>
