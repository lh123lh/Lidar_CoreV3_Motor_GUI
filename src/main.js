import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
// import { useDark, useToggle } from '@vueuse/core'
import SvgIcon from './components/iconfont/SvgIcon.vue'
import './assets/iconfont/iconfont.js'
import router from './router'
import i18n from './locals'
import VWave from 'v-wave'
import VueApexCharts from 'vue3-apexcharts';

const app = createApp(App)
app.use(ElementPlus);
app.use(router);
app.use(i18n);
app.use(VWave);
app.use(VueApexCharts);
app.component('SvgIcon', SvgIcon)
app.mount("#app")
