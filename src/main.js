import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import SvgIcon from './components/iconfont/SvgIcon.vue'
import './assets/iconfont/iconfont.js'
import router from './router'
import i18n from './locals'
import VWave from 'v-wave'
import { createPinia } from "pinia";

const pinia = createPinia()
const app = createApp(App)
app.use(ElementPlus);
app.use(router);
app.use(i18n);
app.use(VWave);
app.use(pinia);
app.component('SvgIcon', SvgIcon)
app.mount("#app")
