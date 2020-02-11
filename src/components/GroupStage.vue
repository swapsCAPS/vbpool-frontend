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
    isDisabled: Boolean,
    group:      Object,
  },

  watch: {
    isDisabled () {
      console.log(this.isDisabled)
    },
  },
}
</script>

<style scoped>
.group {
  display: flex;
  flex-direction: column;
  margin-right: .2rem;
}
.group div span {
  float: left;
  display: inline-block;
  vertical-align: middle;
  line-height: 1rem;
}

.group div input {
  float: right;
  margin-left: 1.0rem;
  width: 1rem;
  height: 1rem;
  text-align: center;
  font-size: 18pt;
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
