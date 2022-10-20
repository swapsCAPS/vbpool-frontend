<template lang="pug">
.section-container
  .section-header
    h4
      span.text {{ text || "Main text" }}
      span {{ ` ` }}
      span.subtext {{ subtext }}
  .section-content
    slot(name="content")
</template>

<script>
export default {
  name:  'SectionComponent',
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
  align-items: center;
  padding: 0.1rem 0.3rem;
}

.section-header h4 {
  margin-bottom: 0 !important;
}

.section-header .text {
  font-weight: 600;
}

.section-header .subtext {
  font-weight: 500;
}

.section-content {
  border: 1px solid #ccc;
}

</style>
