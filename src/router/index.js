import { createRouter, createWebHashHistory } from "vue-router";
// import Proxy from '../views/Proxy.vue'


// 2. 定义一些路由
const routes = [
  // { path: '/', component: BayerRawToRgb },
];

// 3. 创建路由实例并传递 `routes` 配置
const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router;
