import { createRouter, createWebHistory } from 'vue-router'
import DevicesView from '@/views/DevicesView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'devices' },
    },
    {
      path: '/devices',
      name: 'devices',
      component: DevicesView,
    },
  ],
})

export default router
