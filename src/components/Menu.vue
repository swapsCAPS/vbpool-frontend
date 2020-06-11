<template lang="pug">
  div
    .row
      .col-6
        .btn-container
          button.mt-5.btn.btn-primary(
            @click="updatePool"
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
    ...mapActions([ 'updatePool' ]),
    discard () {
      const shoulddiscard = confirm('weet je zeker dat je alles wilt wissen?!')
      if (!shoulddiscard) return
      this.$router.push({ path: '/form' })
    },
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
