<template lang="pug">
  div
    div
      .row.mt-3
        .col
          .errors(v-for="error in errors")
            .error - {{ error.message }}
    Disabler(
      :isDisabled="!user.isLoggedIn || !route.params.poolId"
      message="Vul eerst een naam in voor deze pool en klik op \"Aanmaken\""
    )

</template>

<script>
import Disabler from './Disabler'

import { mapActions, mapState } from 'vuex'

export default {
  name: 'Menu',

  components: {
    Disabler,
  },

  props: {
    errors: Array,
  },

  computed: {
    ...mapState([ 'user', 'route' ]),
  },

  methods: {
    ...mapActions([ 'updatePool', 'deletePool' ]),
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
