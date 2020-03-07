<template lang="pug">
  .row
    .col-lg-1
      span {{ game.nr }}
    .col-lg-11
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
.game-input-wrapper {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border: 1px;
}

.game-input-wrapper > span {
}

.select {
  min-width: 20rem;
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
