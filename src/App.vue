<template lang="pug">
  #app
    PoolForm
</template>

<script>
import PoolForm from './components/PoolForm.vue'

export default {
  name: 'App',
  components: {
    PoolForm,
  }
}
</script>

<style>
body {
  margin: 0 !important;
  padding: 0 !important;
  background-color: #fff8f2;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  position: relative;
}
</style>
