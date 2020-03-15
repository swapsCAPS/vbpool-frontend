<template lang="pug">
  .info-input-container
    h1.title
      span {{ title }}:
      span {{ ' ' }}
      span.dots .................................................................................................
    .input-wrapper
      h1.spacer {{ title }}:
      input(
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
    title: String,
    value: String,
  },
}
</script>

<style scoped>
.info-input-container {
  min-width: 10rem;
  position: relative;
  font-family: 'Cormorant Garamond', serif;
  font-weight: 300;
}

.info-input-container .title {
  overflow: hidden;
  white-space: nowrap;
  cursor: default;
}

.info-input-container .dots {
  color: #aaa;
}

.input-wrapper {
  position: absolute;
  display: flex;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
}

.input-wrapper .spacer {
  float: left;
  visibility: hidden;
}

.input-wrapper input {
  position: relative;
  color: black;
  flex-grow: 0.8;
  margin-left: 0.5rem;
  height: 100%;
  background-color: transparent;
  font-size: 22pt !important;
  overflow: hidden;
}

</style>
