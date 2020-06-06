<template lang="pug">
  .row
    .col-12.mt-4
      .a4.card
        Front
    .col-12.mb-4
      .a4.card.back
        Back
    .col-12.mb-4
      .a4.card
        Menu
</template>

<script>
import * as firebase from 'firebase/app'

import Front from './Front.vue'
import Back from './Back.vue'
import Menu from './Menu.vue'

export default {
  name: 'PoolForm',

  components: {
    Front,
    Back,
    Menu,
  },

  computed: {
  },

  watch: {
  },

  mounted () {
  },

  methods: {
  },

}
</script>

<style scoped>
.a4 {
  background: #fcfdff;
  padding-bottom: 3rem;
  padding-left: 15px;
  padding-right: 15px;
}

.back {
  margin-top: 2rem;
}

</style>
