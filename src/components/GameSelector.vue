<template lang="pug">
  .input
    span {{ name }}
    v-select(
      class="select"
      :options="teams"
    )
</template>

<script>
export default {
  name:  'Game',
  props: {
    name:  String,
    teams: Array,
  },
}
</script>

<style scoped>
.input {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.input span {
  padding-left: 0.5rem;
}

.select {
  flex-grow: 1;
  font-size: 1.2rem;
}

.select .vs__search::placeholder,
.select .vs__dropdown-toggle,
.select .vs__selected,
.select .vs__dropdown-menu {
  font-family: 'Gochi Hand', cursive;
  border: 0;
  line-height: 1.5rem;
}

.select .vs__clear,
.select .vs__open-indicator {
  fill: #394066;
  cursor: pointer;
}

</style>
