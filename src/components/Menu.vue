<template lang="pug">
  div
    .row
      .col-6
        .btn-container
          button.mt-5.btn.btn-primary(
            @click="sendPool"
          ) Versturen
      .col-6
        .btn-container
          button.mt-5.btn.btn-danger(
            @click="discard"
          ) Alles wissen
    .row.mt-3
      .col
        .errors(v-for="error in errors")
          .error - {{ error.message }}

</template>

<script>
import Disabler from './Disabler'

import { mapActions, mapMutations } from 'vuex'

export default {
  name: 'Menu',

  components: {
    Disabler,
  },

  props: {
    errors: Array,
  },

  methods: {
    ...mapActions([ 'sendPool' ]),
    ...mapMutations([ 'discard' ]),
  },

}
</script>

<style scoped>
.btn-container {
  display: flex;
  position: relative;
  justify-content: space-around;
}

</style>
