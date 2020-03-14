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
    .row.mt-3
      .col
        .errors(v-for="error in errors")
          .error - {{ error.message }}

    Disabler(:isDisabled="!isLoggedIn")
</template>

<script>
import Disabler from './Disabler'

export default {
  name: 'Menu',

  components: {
    Disabler,
  },

  props: {
    errors: Array,
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
  border-radius: 0;
  border: 1px fill #333;
}

.btn-primary {
  background: #fcfdff;
  color: #333;
  border: 1px solid #333;
}

.btn-primary:hover {
  background-color: #1f8e00 !important;
  border: 1px solid #1f8e00;
}

.btn-primary:focus {
  background-color: #1f8e00 !important;
  border: 1px solid #1f8e00;
}

.btn-danger {
  background-color: #999 !important;
  border: 1px solid #333;
}

.btn-danger:hover {
  background-color: #333 !important;
  border: 1px solid #333;
}

.btn-danger:focus {
  border-color: #333;
  border: 1px solid #333;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
}

.btn-danger:active {
  border-color: #333;
}

.btn {
  border-color: #333;
  box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.2), 0 3px 10px 0 rgba(0, 0, 0, 0.19);
}

.btn:focus {
  border-color: #333;
  box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.2), 0 3px 10px 0 rgba(0, 0, 0, 0.19);
}

.btn:active:focus {
  border-color: #333;
  box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.2), 0 3px 10px 0 rgba(0, 0, 0, 0.19);
}

</style>
