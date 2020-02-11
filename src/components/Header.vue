<template lang="pug">
  .head
    h1 Café de Laurierboom EK 2020 voetbalpool
    h2 {{ title }}
</template>

<script>
export default {
  name:  'Header',
  props: {
    title: String,
  },
}
</script>

<style scoped>
h1, h2, h3 {
  margin: 0;
}
.head {
  font-family: 'Cormorant Garamond', serif;
  font-weight: 300;
}

.head h1 {
  text-align: center;
  margin: 0.2rem 0;
}

.head h2 {
  text-align: center;
  background-color: #333;
  color: #52b352;
  font-style: italic;
}
</style>
