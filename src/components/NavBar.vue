<template lang="pug">
  nav.navbar.navbar-expand-lg.navbar-light.bg-light
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
          a.nav-link(href="#") Jouw Pools
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
.wrapper {
  position: relative;
  overflow: hidden;
  white-space: nowrap;
}

.wrapper .input {
  position: relative;
}

.wrapper .input input {
  background-color: transparent;
}

.wrapper .dots {
  position: absolute;
  top: 0.4rem;
  left: 0;
  bottom: 0;
  right: 0;
}

</style>
