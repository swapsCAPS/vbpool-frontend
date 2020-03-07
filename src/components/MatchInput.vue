<template lang="pug">
  .input-wrapper
    input(
      type="number"
    )
    span.bold -
    input(
      type="number"
    )
</template>

<script>
export default {
  name: 'MatchInput',

  props: {
    title:     String,
    fieldName: String,
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
  position: relative;
  height: 2.5rem;
}

</style>
