<template lang="pug">
  .container
    Header(
      title="Wedstrijdvoorspellingen"
    )
    .explanation
    .points
    .table
    .footer
      h2 UITERLIJK INLEVEREN OP MAANDAG 8 JUNI 2020
</template>

<script>

import Header from './Header'

export default {
  name: 'Back',

  components: {
    Header,
  },

  props: {
    disableInputs: Boolean,
  },

  data: function () {
    return { }
  },

  computed: {
  },

  created () {
  },

  destroyed () {
  },

  methods: { },
}
</script>

<style scoped>
.container {
  padding: 0.5rem;
  height: 100%;
}

.footer {
  position: absolute;
  bottom: 2rem;
  left: 0.5rem;
  right: 0.5rem;
}

.footer h2 {
  margin: 0 auto;
  bottom: 1rem;
  text-align: center;
  background-color: #333;
  color: white;
  font-family: 'Alias', sans-serif;
}
</style>
