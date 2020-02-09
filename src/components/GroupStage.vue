<template lang="pug">
  .group
    h4 {{ `Groep ${group.name}` }}
    div(v-for="team in group.teams")
      span {{ team.team }}
      input(type="number" min="1" max="4")
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

}

.group div input {
  float: right;
  margin-left: 1.0rem;
  width: 1rem;
  height: 1rem;
  text-align: center;
  font-size: 18pt;
  padding: 0;
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
