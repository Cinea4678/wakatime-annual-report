import { createRouter, createWebHashHistory } from "vue-router"

const routes = [
  { path: "/", component: () => import("@/views/HomeView.vue") },
  { path: "/error", component: () => import("@/views/ErrorView.vue") },
  { path: "/report", component: () => import("@/views/report/ReportView.vue") },
]

// 由于我们的服务器不一定支持路由回退，因此就不使用WebHistory了
export default createRouter({
  history: createWebHashHistory(),
  routes,
})
