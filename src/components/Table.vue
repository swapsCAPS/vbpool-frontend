<template lang="pug">
  .table-responsive-md
    table.table
      tr
        th.datum Datum
        th.time tijd
        th.nr nr
        th.match Wedstrijd
        th.half-time rust
        th.end eind
        th.toto toto
      tr(
        v-for="game in games"
        :key="game.nr"
      )
        td.padded {{ game.formattedDate }}
        td.padded {{ game.tijd }}
        td.padded.nr {{ game.nr }}
        td.padded.game-text {{ `${game.naam1} - ${game.naam2}` }}
        td
          MatchInput(
            v-model="form[game.nr].half"
          )
        td
          MatchInput(
            v-model="form[game.nr].end"
          )
        td.toto
          input(
            type="number"
            min="1"
            max="3"
            v-model="form[game.nr].toto"
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
    form:  {
      type:    Array,
      default: () => [],
    },

  },

}
</script>

<style scoped>
table {
  margin-bottom: 2rem;
  background: white !important;
}

.table td {
}

th {
  font-weight: normal;
  text-align: center;
}

input {
  width: 2rem;
  text-align: center;
  background-color: transparent;
}

table td {
  border: 1px solid #333;
  border-collapse: collapse;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  padding: 0;

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
  padding-left: 0.25rem;
}

tr {
  line-height: 1.5rem;
}

th.half-time, th.end {
  width: 3rem;
}

</style>
