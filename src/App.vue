<template lang="pug">
  #app
    PoolForm
</template>

<script>
import PoolForm from './components/PoolForm.vue'

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
h1, h2, h3, h4, p {
  margin: 0;
}
body {
  margin: 0 !important;
  padding: 0 !important;
  background-color: #fff8f2;
  color: #666;
  cursor: default;
}
input {
  font-family: 'Gochi Hand', cursive;
}
#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  position: relative;
}
</style>
