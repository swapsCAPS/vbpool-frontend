<template lang="pug">
  .game-container
    span {{ game.nr }}
    .inputs.m-left-05
      .game-input-wrapper
        span {{ game.naam1 }}
        v-select(
          class="select"
          :options="teams"
        )
      .game-input-wrapper.no-top-border
        span {{ game.naam2 }}
        .select
          v-select(
            :options="teams"
          )
</template>

<script>
export default {
  name:  'Game',
  props: {
    game:  Object,
    teams: Array,
  },
}
</script>

<style >
.game-container {
  display: flex;
  padding: 0 0.2rem;
  flex-direction: row;
  justify-content: space-between;
}

.game-container > span {
  margin-top: 0.3rem;
}

.inputs {
}

.game-input-wrapper {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 12.5rem;
}

.game-input-wrapper > span {
  min-width: 3.5rem;
}

.select {
  width: 9rem;
}

.select .vs__search::placeholder,
.select .vs__dropdown-toggle,
.select .vs__selected,
.select .vs__dropdown-menu {
  font-family: 'Gochi Hand', cursive;
  border: 0;
  line-height: 1.5rem;
  padding: 0;
  margin: 0;
}

.select .vs__clear,
.select .vs__open-indicator {
  fill: #394066;
}

</style>
