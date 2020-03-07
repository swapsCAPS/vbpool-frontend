<template lang="pug">
  table
    tr
      th.datum Datum
      th.time tijd
      th.nr nr
      th.match Wedstrijd
      th.half-time rust
      th.end eind
      th.toto toto
    tr(v-for="game in games")
      td.padded {{ game.formattedDate }}
      td.padded {{ game.tijd }}
      td.padded.nr {{ game.nr }}
      td.padded.game-text {{ `${game.naam1} - ${game.naam2}` }}
      td
        MatchInput()
      td
        MatchInput()
      td.toto
        input(
          type="number"
          min="1"
          max="3"
        )
</template>

<script>
import MatchInput from './MatchInput'

export default {
  name: 'Table',

  components: {
    MatchInput,
  },

  props: {
    games: Array,
  },

  data: function () { },

  computed: {
  },

  created () {
  },

  methods: { },
}
</script>

<style scoped>
table {
  width: 100%;
  margin-bottom: 2rem;
}

.tables th {
  font-weight: normal;
}

.tables input {
  width: 3rem;
  text-align: center;
  background-color: transparent;
}

.tables table, td {
  border: 1px solid black;
  border-collapse: collapse;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;

}

td.game-text {
  max-width: 8.5rem;
}

.toto {
  text-align: center;
}

.toto input {
  position: relative;;
  width: 2rem;
}

th.toto {
  width: 3rem;
}

th.datum, th.time {
  width: 5rem;
}

th.nr {
  width: 3rem;
}

.padded {
  padding-left: 0.5rem;
}

tr {
  line-height: 1.5rem;
}

th.half-time, th.end {
  width: 3rem;
}

</style>
