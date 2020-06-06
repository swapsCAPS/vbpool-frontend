<template lang="pug">
  nav.navbar.navbar-expand-lg.navbar-light.bg-light.vbp-navbar
    a.navbar-brand Laurierboom Voetbalpool
    button.navbar-toggler(
      type="button"
      data-toggle="collapse"
      data-target="#navbarNav"
    )
      span.navbar-toggler-icon
    #navbarNav.collapse.navbar-collapse
      ul.navbar-nav
        li.nav-item
          router-link(to="/your-pools").nav-link(href="#") Jouw Pools
</template>

<script>
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
}
</script>

<style scoped>
.vbp-navbar {
  background-color: #fcfdff !important;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
}
</style>
