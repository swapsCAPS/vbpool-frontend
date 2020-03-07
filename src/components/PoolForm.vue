<template lang="pug">
  .container
    .row
      .col-sm-12.a4.card()
        Front()
      .col-sm-12.a4.card.back()
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

  data: function () {
    return {
    }
  },

  computed: {
  },

  props: {},

  created () {
  },

  mounted () {
  },

  destroyed () {
  },

  methods: {
  },
}
</script>

<style scoped>
.a4 {
  background: #fcfdff;
}

.back {
  margin-top: 2rem;
}

</style>
