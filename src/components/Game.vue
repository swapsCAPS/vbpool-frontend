<template lang="pug">
  .game-container
    span {{ game.nr }}
    .inputs.m-left-05
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
  padding: 0 0.5rem;
  flex-direction: row;
  justify-content: space-between;
}

</style>
