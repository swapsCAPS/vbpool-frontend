<template lang="pug">
  .container
    .row
      .col-12
        #app
          router-view
</template>

<script>
import VueRouter from 'vue-router'

import * as firebase from 'firebase/app'

import 'vue-select/dist/vue-select.css'

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
  name: 'App',
  router,
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

h1, h2, h3, h4, h5, p {
  margin: 0 !important;
}

h3, h4 {
  margin-bottom: 0.25rem !important;
}

p {
  font-size: 12pt !important;
}

body {
  background-color: #fff8f2 !important;
  color: #666 !important;
  cursor: default !important;
}

.container {
  font-family: 'Times New Roman', serif;
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

#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
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

button {
  border-radius: 0 !important;
  border: 1px fill #333 !important;
  font-family: Arial !important;
}

.btn-primary {
  background: #fcfdff !important;
  color: #333 !important;
  border: 1px solid #333 !important;
}

.btn-primary:hover {
  background-color: #1f8e00 !important;
  border: 1px solid #1f8e00;
}

.btn-primary:focus {
  background-color: #1f8e00 !important;
  border: 1px solid #1f8e00;
}

.btn-danger {
  background-color: #999 !important;
  border: 1px solid #333;
}

.btn-danger:hover {
  background-color: #333 !important;
  border: 1px solid #333;
}

.btn-danger:focus {
  border-color: #333;
  border: 1px solid #333;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
}

.btn-danger:active {
  border-color: #333;
}

.btn {
  border-color: #333;
  box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.2), 0 3px 10px 0 rgba(0, 0, 0, 0.19);
}

.btn:focus {
  border-color: #333;
  box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.2), 0 3px 10px 0 rgba(0, 0, 0, 0.19);
}

.btn:active:focus {
  border-color: #333;
  box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.2), 0 3px 10px 0 rgba(0, 0, 0, 0.19);
}

</style>
