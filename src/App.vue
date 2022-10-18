<template lang="pug">
#app
  .d-flex.is-saving-spinner(v-if="isSaving")
    .spinner(v-if="isSaving")
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

import { mapState } from 'vuex'
export default {
  name:     'App',
  computed: {
    ...mapState([ 'isSaving' ]),
  },
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

.is-saving-spinner {
  position: fixed;
  z-index: 9999;
  top: 0.5rem;
  right: 0.5rem;
}

.is-saving-spinner .spinner {
  height: 2.5rem;
  width: 2.5rem;
  border: 6px solid transparent; /* Light grey */
  border-top: 6px solid #52b352; /* Blue */
  border-radius: 60%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

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
  -webkit-appearance: none !important;
  margin: 0 !important;
}

/* Firefox */
input[type=number] {
  -moz-appearance: textfield !important;
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
