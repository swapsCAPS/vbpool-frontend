import { initializeApp } from 'firebase/app'

import { createApp } from 'vue'

import App from './App.vue'
import vSelect from 'vue-select'

import '@fortawesome/fontawesome-free/css/all.css'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.js'

import { firebaseConfig } from './constants'

import { fbAuthObservablePromiseWrapper } from './helpers'

import store from './store'
import router from './router'

const fbApp = initializeApp(firebaseConfig)
console.log('firebaseConfig', firebaseConfig)
if (location.hostname === 'localhost') {
  // const db = getFirestore(fbApp)
  // db.settings({
  // host: 'localhost:8081',
  // ssl:  false,
  // })
}

;(async function go () {
  const user = await fbAuthObservablePromiseWrapper(fbApp)
  if (user) {
    store.commit('setLoggedIn', true)
    store.commit('setUser', user)
  } else {
    store.commit('setLoggedIn', false)
    store.commit('resetUser')
  }

  const app = createApp(App)

  app.use(router)
  app.use(store)

  app.component('v-select', vSelect)

  app.config.productionTip = false

  app.mount('#app')
})()
