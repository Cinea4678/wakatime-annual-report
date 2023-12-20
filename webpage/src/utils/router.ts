import { createRouter, createWebHistory } from "vue-router";

const routes = [
  { path: "/", component: () => import("@/views/HomeView.vue") },
  { path: "/error", component: () => import("@/views/ErrorView.vue") },
  { path: "/report", component: () => import("@/views/report/ReportView.vue") },
];

export default createRouter({
  history: createWebHistory(),
  routes,
});
