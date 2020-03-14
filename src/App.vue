<template lang="pug">
  .container
    .row
      .col-12
        #app
          router-view
</template>

<script>
import VueRouter from 'vue-router'

import 'vue-select/dist/vue-select.css'

import PoolForm from './components/PoolForm.vue'

const routes = [
  { path: '/', redirect: '/form' },
  { path: '/form', component: PoolForm },
]

export default {
  name:       'App',
  router:     new VueRouter({ routes }),
  components: {

    PoolForm,
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css?family=Gochi+Hand&display=swap');
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
  font-family: 'Gochi Hand', cursive !important;
  border: 0 !important;
  font-size: 18pt !important;
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

.border {
  border: 1px solid #ccc;
}

.no-top-border {
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
