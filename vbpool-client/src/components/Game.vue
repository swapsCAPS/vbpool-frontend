<template lang="pug">
.game
  span.nr {{ `${game.nr}:` }}
  .inputs
    GameSelector(
      :name="game.naam1"
      :teams="teams"
      v-model="value[0]"
    ).vbp-border
    GameSelector(
      :name="game.naam2"
      :teams="teams"
      v-model="value[1]"
    ).vbp-border.vbp-no-top-border
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

.nr {
}

.inputs {
  margin-left: 0.5rem;
  display: flex;
  flex-direction: column;
  width: 100%;
}
</style>
