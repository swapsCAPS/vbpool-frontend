<template lang="pug">
  .input-wrapper
    input(
      type="number"
      v-model.number="value[0]"
    )
    span -
    input(
      type="number"
      v-model.number="value[1]"
    )
</template>

<script>
export default {
  name: 'MatchInput',

  props: {
    title: String,
    value: {
      type:    Array,
      default: () => [],
    },
  },
}
</script>

<style scoped>
.input-wrapper {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
}

.input-wrapper input {
  width: 2.5rem;
  text-align: center;
}

.input-wrapper span {
  font-weight: bold;
}

</style>
