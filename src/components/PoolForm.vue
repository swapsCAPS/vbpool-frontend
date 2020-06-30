<template lang="pug">
  .row
    .col-12
      .a4.card
        Front
    .col-12.mb-4
      .a4.card.back
        Back
</template>

<script>
import Front from './Front.vue'
import Back from './Back.vue'
import Menu from './Menu.vue'
import _ from 'lodash'

import { mapState }  from 'vuex'

export default {
  name:       'PoolForm',
  components: {
    Front,
    Back,
    Menu,
  },
  computed: {
    ...mapState([ 'form' ]),
  },
  methods: {
    debouncedUpdatePool: _.debounce(function () { this.$store.dispatch({ type: 'updatePool' }) }, 2500),
  },
  watch: {
    // NOTE we use a watch because adding all fields of the form to vuex is very tedious to do even with
    // vuex-map-fields...
    form: {
      handler () {
        this.debouncedUpdatePool()
      },
      deep: true,
    },
  },
}
</script>

<style scoped>
.back {
  margin-top: 2rem;
}

</style>
