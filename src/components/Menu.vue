<template lang="pug">
  div
    .row
      .col-6
        .btn-container
            button.mt-5.btn.btn-primary(
              @click="$emit('send')"
            ) Versturen
      .col-6
        .btn-container
          button.mt-5.btn.btn-danger(
            @click="$emit('discard')"
          ) Alles wissen
</template>

<script>
export default {
  name: 'Menu',

  components: {
  },

  props: {
  },

}
</script>

<style scoped>
.btn-container {
  display: flex;
  justify-content: space-around;
  font-family: Arial;
}
.btn-container button {
  min-width: 8rem;
}
</style>
