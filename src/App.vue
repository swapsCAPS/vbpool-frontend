<template lang="pug">
  #app
    .container
      .row
        .col-12
          NavBar
    .container
      .row
        .col-12
          router-view
</template>

<script>
import VueRouter from 'vue-router'

import * as firebase from 'firebase/app'

import 'vue-select/dist/vue-select.css'

import NavBar from './components/NavBar.vue'
import PoolForm from './components/PoolForm.vue'
import SignIn from './components/SignIn.vue'
import VerifyEmail from './components/VerifyEmail.vue'

import store from './store'

const routes = [
  { path: '/', name: 'root', redirect: '/form' },
  {
    path:        '/signin',
    name:        'signin',
    component:   SignIn,
    beforeEnter: (to, from, next) => {
      if (store.state.user.isLoggedIn) return next({ name: 'form' })
      next()
    },
  },
  { path: '/form', name: 'form', component: PoolForm },
  { path: '/verify-email', name: 'verify-email', component: VerifyEmail },
]

const router = new VueRouter({ routes })

router.beforeEach((to, from, next) => {
  if (to.name === 'signin' || to.name === 'verify-email') return next()
  if (!store.state.user.isLoggedIn) return next({ name: 'signin' })
  next()
})

export default {
  name:       'App',
  router,
  components: {
    NavBar,
  },
  mounted () {
    firebase.auth().onAuthStateChanged((user) => {
      if (user) {
        store.commit('setLoggedIn', true)
        store.commit('setEmail', user.email)
        router.push('form')
      } else {
        store.commit('setLoggedIn', false)
        store.commit('setEmail', '')
        router.push('signin')
      }
    })
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css?family=Delius&display=swap');
@import url('https://fonts.googleapis.com/css?family=Cormorant+Garamond:300,300i,400&display=swap');

body {
  background-color: #fff8f2 !important;
  color: #666 !important;
  cursor: default !important;
  font-family: 'Times New Roman', serif !important;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

input {
  font-family: 'Delius', cursive !important;
  -webkit-appearance: none !important;
  -webkit-focus-ring-color: none !important;
  border-radius: 0;
  border: 0;
}

/* Disable number spinners */
/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}

.card {
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
}

.vbp-border {
  border: 1px solid #ccc;
}

.vbp-no-top-border {
  border-top: none !important;
}

.bold {
  font-weight: bold;
}

</style>
