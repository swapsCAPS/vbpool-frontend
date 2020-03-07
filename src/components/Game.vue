<template lang="pug">
  .row
    .col-xs-1
      span {{ `${game.nr}:` }}
    .col-xs-10
      .row
        .col-xs-2
          span {{ game.naam1 }}
        .col-xs-9.float-right
          v-select(
            class="select"
            :options="teams"
          )
      .row
        .col-xs-2
          span {{ game.naam2 }}
        .col-xs-9.float-right
          v-select(
            class="select"
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
  border: 1px #333;
}

.float-right {
  float: right;
}

.select {
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
