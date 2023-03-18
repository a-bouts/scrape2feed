import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'

import App from './App.vue'
import Feeds from './components/Feeds.vue'
import Feed from './components/Feed.vue'

import 'bulma/css/bulma.css'

import '@fortawesome/fontawesome-free/css/all.css'
import '@fortawesome/fontawesome-free/js/all.js'

const routes = [
    { path: '/', component: Feeds, props: true},
    { path: '/new', component: Feed, props: (route) => ({ url: route.query.url })}
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
  })
  
  
createApp(App).use(router).mount('#app')
