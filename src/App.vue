<template lang="pug">
  .container
    .row
      .col-12
        #app
          PoolForm
</template>

<script>
import PoolForm from './components/PoolForm.vue'

import 'vue-select/dist/vue-select.css'

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

</style>
