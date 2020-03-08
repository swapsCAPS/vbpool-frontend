<template lang="pug">
  .input-wrapper
    input(
      type="number"
    )
    span -
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
  width: 2.5rem;
  text-align: center;
}

.input-wrapper span {
  font-weight: bold;
}

</style>
