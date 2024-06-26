import { createRouter, createWebHashHistory } from "vue-router";
// import Proxy from '../views/Proxy.vue'
import Motor_main from '../views/Motor_main.vue'
import speedMonitor from '../views/Speed_monitor.vue'
import stressTest from '../views/Stress_test.vue'
import setting from '../views/Setting.vue'


// 2. 定义一些路由
const routes = [
  {
    path: '/',
    name: 'home',
    meta: {
      //	当前页面要不要缓存
      keepAlive: true,
      //	当前页面层级
      deepth: 1,
    },
    component: Motor_main
  },
  {
    path: '/speedMonitor',
    name: 'monitor',
    meta: {
      //	当前页面要不要缓存
      keepAlive: true,
      //	当前页面层级
      deepth: 2,
    },
    component: speedMonitor
  },
  {
    path: '/stressTest',
    name: 'stressTest',
    meta: {
      //	当前页面要不要缓存
      keepAlive: true,
      //	当前页面层级
      deepth: 3,
    },
    component: stressTest
  },
  {
    path: '/setting',
    name: 'setting',
    meta: {
      //	当前页面要不要缓存
      keepAlive: true,
      //	当前页面层级
      deepth: 4,
    },
    component: setting
  },
];

// 3. 创建路由实例并传递 `routes` 配置
const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router;
