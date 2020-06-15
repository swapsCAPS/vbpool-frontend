<template lang="pug">
  #app
    .container
      .row
        .col-12
          NavBar
    .container.mt-4
      .row
        .col-12
          router-view
</template>

<script>
import 'vue-select/dist/vue-select.css'

import NavBar from './components/NavBar.vue'

import store from './store'
import router from './router'

export default {
  name:       'App',
  components: {
    NavBar,
  },
  router,
  store,
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

.a4 {
  background-color: #fcfdff;
  padding-bottom: 3rem;
  padding-left: 15px;
  padding-right: 15px;
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
