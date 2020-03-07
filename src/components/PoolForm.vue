<template lang="pug">
  .pool-forms
    .a4.card()
      Front()
    .a4.card.back()
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
.pool-forms {
  padding: 2rem 0;
}

.a4 {
  position: relative;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  min-width: 740px;
  max-width: 1000px;
  margin-left: auto;
  margin-right: auto;
  background: #fcfdff;
  width: 100%;
  height: 100%;
}

.back {
  margin-top: 2rem;
}

</style>
