<template lang="pug">
.ml-2.mr-2.wrapper.flex-fill
  h3.dots ..............................................................................................................................
  h3.d-flex.input
    input.flex-fill(
      :type="inputType"
      :value="value"
      @input="$emit('input', $event.target.value)"
    )
</template>

<script>
export default {
  name: 'InfoInput',

  props: {
    inputType: {
      type:    String,
      default: () => 'text',
    },
    editable: Boolean,
    title:    String,
    value:    String,
  },
}
</script>

<style scoped>
.wrapper {
  position: relative;
  overflow: hidden;
  white-space: nowrap;
}

.wrapper .input {
  position: relative;
}

.wrapper .input input {
  background-color: transparent;
}

.wrapper .dots {
  position: absolute;
  top: 0.4rem;
  left: 0;
  bottom: 0;
  right: 0;
}

</style>
