import 'firebase/auth'
import 'firebase/firestore'

import firebase from 'firebase/app'

import Vue from 'vue'
import Vuex from 'vuex'

import App from './App.vue'
import vSelect from 'vue-select'
import VueRouter from 'vue-router'

import '@fortawesome/fontawesome-free/css/all.css'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.js'

import { firebaseConfig } from './constants'
import { sync } from 'vuex-router-sync'

import { fbAuthObservablePromiseWrapper } from './helpers'

import store from './store'
import router from './router'
sync(store, router)

firebase.initializeApp(firebaseConfig)
const db = firebase.firestore()
if (location.hostname === 'localhost') {
  db.settings({
    host: 'localhost:8081',
    ssl:  false,
  })
}

Vue.use(VueRouter)
Vue.use(Vuex)

Vue.component('v-select', vSelect)

Vue.config.productionTip = false

;(async function go () {
  const user = await fbAuthObservablePromiseWrapper()
  new Vue({
    async created () {
      if (user) {
        store.commit('setLoggedIn', true)
        store.commit('setUser', user)
      } else {
        store.commit('setLoggedIn', false)
        store.commit('resetUser')
      }
    },
    render: h => h(App),
  }).$mount('#app')
})()
