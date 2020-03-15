<template lang="pug">
  .game
    span {{ `${game.nr}:` }}
    .inputs
      GameSelector(
        :name="game.naam1"
        :teams="teams"
        v-model="value[0]"
      ).border
      GameSelector(
        :name="game.naam2"
        :teams="teams"
        v-model="value[1]"
      ).border.no-top-border
</template>

<script>
import GameSelector from './GameSelector'
export default {
  name:       'Game',
  components: {
    GameSelector,
  },
  props: {
    game:  Object,
    teams: Array,
    value: {
      type:    Array,
      default: () => [],
    },
  },
}
</script>

<style >
.game {
  position: relative;
  display: flex;
  flex-direction: row;
  align-items: baseline;
  margin: 0.2rem;
}

.inputs {
  margin-left: 0.5rem;
  display: flex;
  flex-direction: column;
  width: 100%;
}
</style>
