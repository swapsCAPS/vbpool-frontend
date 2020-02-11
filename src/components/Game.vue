<template lang="pug">
  .game-container
    span {{ game.nr }}
    div.inputs
      .game-input-wrapper
        span {{ game.naam1 }}
        input(type="text")
      .game-input-wrapper.no-top-border
        span {{ game.naam2 }}
        input(type="text")
</template>

<script>
export default {
  name:  'Game',
  props: {
    game: Object,
  },
}
</script>

<style scoped>
.game-container {
  display: flex;
  width: 100%;
  padding: 0 0.5rem;
}

.inputs {
  margin-left: 0.5rem;
  width: 100%;
}
</style>
