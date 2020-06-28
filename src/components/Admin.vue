<template lang="pug">
  .a4.card
    .row
      .col
        Header(
          title="Administratie"
        )
    .row
      .col
        div.list-group
          .list-group-item.list-group-item-action(
            v-for="pool of user.pools"
            :href="`#/your-pools/${pool.id}`"
          )
            a {{ pool.name }}
            button.btn.float-right dingen

</template>

<script>
import Header from './Header'

import { mapState } from 'vuex'
export default {
  name:       'Admin',
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
