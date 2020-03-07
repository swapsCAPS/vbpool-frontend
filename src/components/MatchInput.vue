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
}

.input-wrapper input {
  position: relative;
  border: 0;
  background-color: red;
  font-size: 16pt;
  max-width: 1rem;
  text-align: center;

}

</style>
