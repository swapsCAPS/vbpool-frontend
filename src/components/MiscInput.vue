<template lang="pug">
  .input
    span {{ text }}
    input(
      :type="inputType"
      :value="value"
      @input="$emit('input', $event.target.value)"
    )
</template>

<script>
export default {
  name:  'MiscInput',
  props: {
    text:      String,
    value:     [ String, Number ],
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
