<template lang="pug">
  .section-container
    .section-header
      h4.text {{ text || "Main text" }}
      h4.subtext {{ subtext }}
    .section-content
      slot(name="content")
</template>

<script>
export default {
  name:  'Section',
  props: {
    text:    String,
    subtext: String,
  },
}
</script>

<style scoped>
.section-container {
  margin-top: 0.8rem;
  min-height: 10px;
}

.section-header {
  background-color: #333;
  color: #fcfdff;
  font-family: 'Cormorant Garamond', serif;
  display: flex;
}

.section-header .text {
  margin-left: 0.3rem;
  margin-top: 0.1rem;
  margin-bottom: 0.1rem;
  font-weight: 600;
}

.section-header .subtext {
  font-weight: 500;
  margin-left: 1rem;
}

.section-content {
  border: 1px solid #ccc;
}

</style>
