import * as firebase from 'firebase/app'
import 'firebase/auth'
import 'firebase/firestore'

import Vue from 'vue'

import App from './App.vue'
import vSelect from 'vue-select'
import VueRouter from 'vue-router'

import 'bootstrap/dist/css/bootstrap.min.css'

import { firebaseConfig } from './constants'

firebase.initializeApp(firebaseConfig)

Vue.use(VueRouter)

Vue.component('v-select', vSelect)

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
}).$mount('#app')
