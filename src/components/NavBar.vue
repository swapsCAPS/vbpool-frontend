<template lang="pug">
  nav.navbar.navbar-expand-lg.navbar-light.bg-light.vbp-navbar
    router-link(to="/").navbar-brand Laurierboom Voetbalpool
    button.navbar-toggler(
      type="button"
      data-toggle="collapse"
      data-target="#navbarNav"
    )
      span.navbar-toggler-icon
    #navbarNav.collapse.navbar-collapse
      ul.navbar-nav
        li.nav-item
          router-link(to="/your-pools").nav-link Jouw Pools
        li.nav-item(v-if="user.role == 'admin'")
          router-link(to="/admin").nav-link Administratie
</template>

<script>
import { mapState } from 'vuex'

export default {
  name: 'InfoInput',

  props: {
    inputType: {
      type:    String,
      default: () => 'text',
    },
    editable: Boolean,
    title:    String,
    value:    String,
  },

  computed: {
    ...mapState([ 'user' ]),
  },
}
</script>

<style scoped>
.vbp-navbar {
  background-color: #fcfdff !important;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
}
</style>
