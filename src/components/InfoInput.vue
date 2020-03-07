<template lang="pug">
  .info-input-container
    h1.title {{ title }}: .................................................................................................
    .input-wrapper
      h1.spacer {{ title }}:
      input(
        type="text"
      )
</template>

<script>
export default {
  name: 'InfoInput',

  props: {
    title:     String,
    fieldName: String,
  },

  watch: {
  },
}
</script>

<style scoped>
.info-input-container {
  position: relative;
  margin-top: 0.5rem;
  padding-right: 1rem;
  font-family: 'Cormorant Garamond', serif;
  font-weight: 300;
}

.info-input-container .title {
  overflow: hidden;
  white-space: nowrap;
  cursor: default;
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
  bottom: 0.6rem;
  border: 0;
  height: 100%;
  background-color: transparent;
  font-size: 22pt;

}

</style>
