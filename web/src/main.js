import Vue from 'vue'
import VueRouter from 'vue-router'
import App from './App.vue'

import Feeds from './components/Feeds.vue'
import Feed from './components/Feed.vue'

import 'bulma/css/bulma.css'

Vue.use(VueRouter)

Vue.config.productionTip = false

const routes = [
  { path: '/', component: Feeds, props: true},
  { path: '/new', component: Feed, props: (route) => ({ url: route.query.url })}
]

const router = new VueRouter({
  routes
})

new Vue({
  render: h => h(App),
  router
}).$mount('#app')
