import * as firebase from 'firebase/app'
import 'firebase/auth'
import 'firebase/firestore'

import Vue from 'vue'
import Vuex from 'vuex'

import App from './App.vue'
import vSelect from 'vue-select'
import VueRouter from 'vue-router'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.js'

import { firebaseConfig } from './constants'
import { sync } from 'vuex-router-sync'

import router from './router'
import store from './store'
import { fbAuthObservablePromiseWrapper } from './helpers'

sync(store, router)

firebase.initializeApp(firebaseConfig)

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
        store.commit('setEmail', user.email)
        const token = await firebase.auth().currentUser.getIdTokenResult()
        store.commit('setRole', token.claims.role || 'user')
      } else {
        store.commit('setLoggedIn', false)
        store.commit('setEmail', '')
      }
    },
    router,
    store,
    render: h => h(App),
  }).$mount('#app')
})()
