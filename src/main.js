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

import store from './store'

firebase.initializeApp(firebaseConfig)

Vue.use(VueRouter)
Vue.use(Vuex)

Vue.component('v-select', vSelect)

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  store,
}).$mount('#app')
