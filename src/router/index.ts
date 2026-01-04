import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '../views/Dashboard.vue'
import LibraryView from '../views/LibraryView.vue'
import AlbumDetailView from '../views/AlbumDetailView.vue'
import SettingsView from '../views/SettingsView.vue'

const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: Dashboard
  },
  {
    path: '/library',
    name: 'Library',
    component: LibraryView
  },
  {
    path: '/album/:id',
    name: 'AlbumDetail',
    component: AlbumDetailView
  },
  {
    path: '/albums/edit',
    name: 'MultiAlbumEdit',
    component: AlbumDetailView
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
