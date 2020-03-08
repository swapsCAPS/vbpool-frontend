<template lang="pug">
  .game
    span {{ `${game.nr}:` }}
    .inputs
      GameSelector(
        :name="game.naam1"
        :teams="teams"
      ).border
      GameSelector(
        :name="game.naam2"
        :teams="teams"
      ).input.border.no-top-border
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

.input {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.select {
  flex-grow:1;
}

.select .vs__search::placeholder,
.select .vs__dropdown-toggle,
.select .vs__selected,
.select .vs__dropdown-menu {
  font-family: 'Gochi Hand', cursive;
  border: 0;
  line-height: 1.5rem;
}

.select .vs__clear,
.select .vs__open-indicator {
  fill: #394066;
  cursor: pointer;
}

</style>
