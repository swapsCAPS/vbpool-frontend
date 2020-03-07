<template lang="pug">
  .group
    h4 {{ `Groep ${group.name}` }}
    div(v-for="team, index in group.teams")
      span {{ team.team }}
      input(:class="{ 'no-top-border': index !== 0 }" type="number" min="1" max="4")
</template>

<script>
export default {
  name: 'GroupStage',

  props: {
    group: Object,
  },

  watch: {
  },
}
</script>

<style scoped>
.group {
  display: flex;
  flex-direction: column;
  padding: 0 0.5rem;
  min-width: 5rem;
  max-width: 12rem;
}

.group div {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.group div span {
  float: left;
  display: inline-block;
  vertical-align: middle;
  min-width: 7rem;
  line-height: 1.5rem;
}

.group div input {
  float: right;
  width: 1.5rem;
  height: 1.5rem;
  text-align: center;
  padding: 0;
  border: 1px solid #ccc;
}

.no-top-border {
  border-top: none !important;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
/* Firefox */
input[type=number] {
  -moz-appearance:textfield;
}
</style>
