<template lang="pug">
  .input
    span {{ name }}
    v-select(
      class="select"
      :options="teams"
      :value="value"
      :searchable="false"
      @input="input"
    )
</template>

<script>
export default {
  name:  'Game',
  props: {
    name:  String,
    teams: Array,
    value: String,
  },

  methods: {
    // v-model was acting up
    input (value) {
      this.$emit('input', value)
    },
  },
}
</script>

<style>
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
  font-size: 14pt;
  font-family: 'Delius', cursive !important;
  -webkit-appearance: none;
  -webkit-border-radius: 0px;
}

.select .vs__dropdown-toggle {
  border: 0 !important;
}

.select .vs__search::placeholder,
.select .vs__dropdown-toggle,
.select .vs__selected,
.select .vs__dropdown-menu {
  line-height: 1.5rem;
}

.select .vs__clear,
.select .vs__open-indicator {
  fill: #394066;
  cursor: pointer;
}

</style>
