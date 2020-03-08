<template lang="pug">
  .input
    span {{ text }}
    input(
      :type="inputType"
    )
</template>

<script>
export default {
  name:  'MiscInput',
  props: {
    text:      String,
    inputType: {
      type:    String,
      default: 'text',
    },
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

.input input {
  width: 0;
  margin-left: 0.5rem;
  flex-grow: 1;
  height: 1.225em;
}

</style>
