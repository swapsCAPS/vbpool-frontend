<template lang="pug">
  .row
    .col-12.mt-4.a4.card()
      Front()
    .col-12.mb-4.a4.card.back()
      Back()
</template>

<script>

import Front from './Front.vue'
import Back from './Back.vue'

export default {
  name: 'PoolForm',

  components: {
    Front,
    Back,
  },

}
</script>

<style scoped>
.a4 {
  background: #fcfdff;
  padding-bottom: 3rem;
}

.back {
  margin-top: 2rem;
}

</style>
