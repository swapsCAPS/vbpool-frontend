<template lang="pug">
  #app
    PoolForm
</template>

<script>
import PoolForm from './components/PoolForm.vue'

import 'vue-select/dist/vue-select.css'
import './assets/bootstrap.min.css'

export default {
  name:       'App',
  components: {

    PoolForm,
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css?family=Gochi+Hand&display=swap');
@import url('https://fonts.googleapis.com/css?family=Cormorant+Garamond:300,300i,400&display=swap');

h1, h2, h3, h4, h5, p {
  margin: 0;
}

h3, h4 {
  margin-bottom: 0.25rem;
}

p {
  font-size: 12pt;
}

body {
  background-color: #fff8f2;
  color: #666;
  cursor: default;
}

input {
  padding: 0;
  font-family: 'Gochi Hand', cursive;
  border: 0;
  font-size: 18pt;
  max-height: 1.5rem
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
  position: relative;
}

.card {
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
}

.no-top-border {
  border-top: none !important;
}

.bold {
  font-weight: bold;
}

</style>
