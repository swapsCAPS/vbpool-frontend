<template lang="pug">
  div
    .info-input-container
      h3
        span {{ `${title}: ` }}
      .ml-1.wrapper(v-if="editable")
        h3.dots .................................................................................................
          input(
            :type="inputType"
            :value="value"
            @input="$emit('input', $event.target.value)"
          )
      .ml-1.wrapper(v-else)
        h3.email-text {{ value }}
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
.info-input-container {
  min-width: 3rem;
  display: flex;
  display:-webkit-box;
  justify-content: space-between;
  align-items: baseline;
  padding-top: 0.4rem;
}

.email-text {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
.wrapper {
  flex-grow: 1;
  position: relative;
  overflow: hidden;
  white-space: nowrap;
  margin-top: 0.4rem;
}

.wrapper input {
  position: absolute;
  top: -0.4rem;
  left: 0;
  width: 100%;
  bottom: 0;
  background-color: transparent;

}

.dots {
  color: #;
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
  overflow: hidden;
}

</style>
