<template lang="pug">
  .a4.card
    .row
      .col
        Header(
          title="Jouw pools"
        )
    .row
      .col

</template>

<script>
import Header from './Header'

export default {
  name:       'PoolForm',
  components: {
    Header,
  },
}
</script>

<style scoped>
.back {
  margin-top: 2rem;
}

</style>
