<template lang="pug">
  .a4.card
    .row
      .col
        Header(
          title="Jouw pools"
        )
    .row
      .col
        .list-group.list-group-flush(v-if="user.pools.length")
          router-link.list-group-item.list-group-item-action(
            v-for="pool of user.pools"
            :to="`/form/${pool.id}`"
            :class="{ disabled: pool.flags.isPaid }"
          )
            .d-flex.align-items-baseline.justify-content-between
              span {{ pool.meta.poolName }}
              div(
                v-if="!pool.flags.isPaid"
              )
                .btn.btn-outline-primary.mr-1
                  span.fa.fa-edit
                .btn.btn-outline-danger
                  span.fa.fa-trash
              div(v-else)
                .btn.btn.btn-outline-secondary.disabled
                  span.fa.fa-money-bill.mr-1
                  span.fa.fa-user-check
        .text-center.mt-5(v-else)
          h3
            span Je hebt nog geen pools.
            span {{ ` ` }}
            router-link(
              :to="`/form`"
            ) Klik hier om een pool aan te maken.

</template>

<script>
import Header from './Header'

import { mapState } from 'vuex'
export default {
  name:       'PoolForm',
  components: {
    Header,
  },
  computed: {
    ...mapState([ 'user' ]),
  },
  created () {
    this.$store.dispatch({ type: 'listUserPools' })
  },

}
</script>

<style scoped>
.back {
  margin-top: 2rem;
}

</style>
