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
            :class="{ disabled: pool.isPayed }"
          )
            .d-flex.align-items-baseline.justify-content-between
              span {{ pool.meta.poolName }}
              div(
                v-if="!pool.isPayed"
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
