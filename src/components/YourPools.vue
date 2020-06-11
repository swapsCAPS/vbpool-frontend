<template lang="pug">
  .a4.card
    .row
      .col
        Header(
          title="Jouw pools"
        )
    .row
      .col
        .list-group.list-group-flush
          router-link.list-group-item.list-group-item-action(
            v-for="pool of user.pools"
            :to="`/form/${pool.id}`"
            :class="{ disabled: pool.isPayed }"
          )
            a
              span  {{ pool.form.page1.meta.poolName }}

</template>

<script>
import Header from './Header'

import { mapState, mapActions } from 'vuex'
export default {
  name:       'PoolForm',
  components: {
    Header,
  },
  computed: {
    ...mapState([ 'user' ]),
  },
  methods: {
    ...mapActions([ 'fetchUserPools' ]),
  },
  created () {
    this.fetchUserPools()
  },

}
</script>

<style scoped>
.back {
  margin-top: 2rem;
}

</style>
