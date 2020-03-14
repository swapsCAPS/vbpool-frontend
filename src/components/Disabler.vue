<template lang="pug">
  .row
    .col.disabler(
      :class="{ isDisabled }"
      @click="alert"
    )
</template>

<script>

export default {
  name: 'Disabler',

  props: {
    isDisabled: Boolean,
  },

  methods: {
    alert () {
      window.alert('Vul AUB je email adres in')
    },
  },

}
</script>

<style scoped>
.isDisabled {
  background-color: #0000000f;
  display: block !important;
}

.disabler {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: none;
}

</style>
