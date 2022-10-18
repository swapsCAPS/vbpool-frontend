<template lang="pug">
.row
  .col-12
    .a4.card
      FrontComponent
  .col-12.mb-4
    .a4.card.back
      BackComponent
</template>

<script>
import FrontComponent from './FrontComponent.vue'
import BackComponent from './BackComponent.vue'
import MenuComponent from './MenuComponent.vue'
import _ from 'lodash'

import { mapState }  from 'vuex'

export default {
  name:       'PoolForm',
  components: {
    FrontComponent,
    BackComponent,
    MenuComponent,
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
