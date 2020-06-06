<template lang="pug">
  .row
    .col-12
      .a4.card
        Front
    .col-12.mb-4
      .a4.card.back
        Back
    .col-12.mb-4
      .a4.card
        Menu
</template>

<script>
import Front from './Front.vue'
import Back from './Back.vue'
import Menu from './Menu.vue'

export default {
  name:       'PoolForm',
  components: {
    Front,
    Back,
    Menu,
  },
}
</script>

<style scoped>
.back {
  margin-top: 2rem;
}

</style>
