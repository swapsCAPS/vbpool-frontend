<template lang="pug">
  .d-inline-flex.align-items-baseline
    h3
      span {{ `${title}: ` }}
    .ml-1(v-if="editable")
      .wrapper
        h3 .................................................................................................
          input(
            :type="inputType"
            :value="value"
            @input="$emit('input', $event.target.value)"
          )
    .ml-1(v-else)
      h3 {{ value }}
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
  margin-top: 0.4rem;
  max-width: 15rem;
}

.wrapper input {
  position: absolute;
  top: -0.4rem;
  left: 0;
  bottom: 0;
  background-color: transparent;
}

</style>
