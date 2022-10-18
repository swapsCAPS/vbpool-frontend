import { initializeApp } from 'firebase/app'
import { getFirestore } from 'firebase/firestore'

import { createApp } from 'vue'

import App from './App.vue'
import vSelect from 'vue-select'

import '@fortawesome/fontawesome-free/css/all.css'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.js'

import { firebaseConfig } from './constants'

import { fbAuthObservablePromiseWrapper } from './helpers'

/* eslint-disable-next-line */
const fbApp = initializeApp(firebaseConfig)

/* eslint-disable-next-line */
getFirestore(fbApp)

/* eslint-disable-next-line */
import router from './router'
/* eslint-disable-next-line */
import store from './store'

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
