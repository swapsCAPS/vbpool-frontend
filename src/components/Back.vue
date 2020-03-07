<template lang="pug">
  div
    .row
      .col-sm-12
        Header(
          title="Wedstrijdvoorspellingen"
        )
    .row
      .col-sm-12
        .explanation
          h3 Uitleg
          p
            p
              span {{ `Vul hieronder voor alle wedstrijden jouw uitslagen in. ` }}
              span.bold Ook daar waar de teams nog niet bekend zijn.
            p (Ook al heb je een ander team op die plaat dan kan je uitslag nog steeds goed zijn)
            p De uitslag hoeft onderling niet te kloppen. Je krijgt punten voor elk vak dat achteraf juist blijkt te zijn ingevuld.
            p Bij 'toto' vul je een 1 in voor winst linker team, een 2 voor winst rechter team en een 3 voor een gelijkspel.
        .warning.m-top-05
          h3 Alle uitslagen, ook de toto, gelden na 90 minuten voetbal!
          span (plus de eventuele blessuretijd)
        .points.m-top-05
          h3 Punten
          p Ruststand goed: 2 pnt, Eindstand goed: 3 pnt, Toto goed: 4 pnt. Totaal aantal doelpunten op één dag goed: 5 pnt.
    .row
      .col-xs-12
        .tables
          .row
            .col-xs-12.col-lg-6
              table
                tr
                  th Datum
                  th tijd
                  th nr
                  th Wedstrijd
                  th rust
                  th eind
                  th toto
                tr(v-for="game in games.left")
                  td {{ game.formattedDate }}
                  td {{ game.tijd }}
                  td {{ game.nr }}
                  td.game-text {{ `${game.naam1} - ${game.naam2}` }}
                  td
                    MatchInput(
                    )
                  td
                    MatchInput(
                    )
                  td.toto
                    input(
                      type="number"
                      min="1"
                      max="3"
                    )
            .col-xs-12.col-lg-6
              table
                tr
                  th Datum
                  th tijd
                  th nr
                  th Wedstrijd
                  th rust
                  th eind
                  th toto
                tr(v-for="game in games.right")
                  td {{ game.formattedDate }}
                  td {{ game.tijd }}
                  td {{ game.nr }}
                  td.game-text {{ `${game.naam1} - ${game.naam2}` }}
                  td
                    MatchInput(
                    )
                  td
                    MatchInput(
                    )
                  td.toto
                    input(
                      type="number"
                      min="1"
                      max="3"
                    )
    .row
      .col-xs-12
        .footer
          h2 UITERLIJK INLEVEREN OP MAANDAG 8 JUNI 2020
</template>

<script>
import Header from './Header'
import MatchInput from './MatchInput'

import allGames from '../assets/games.json'

export default {
  name: 'Back',

  components: {
    Header,
    MatchInput,
  },

  props: {
  },

  data: function () {
    return {
      allGames,
    }
  },

  computed: {
    games () {
      const { games } = allGames
      return {
        left:  games.slice(0, (games.length / 2) + 1),
        right: games.slice(-games.length / 2),
      }
    },
  },

  created () {
  },

  destroyed () {
  },

  methods: { },
}
</script>

<style>
.container {
  font-family: 'Times New Roman', serif;
}

.explanation {
  margin-top: 1rem;
}

.warning {
  text-align: center;
}

.tables {
  margin-top: 1rem;
}

.tables table {
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

.footer {
  margin-top: 2rem;
  bottom: 2rem;
  left: 0.5rem;
  right: 0.5rem;
}

.toto {
  text-align: center;
}

.toto input {
  position: relative;;
  width: 2rem%;
}

.footer h2 {
  margin: 0 auto;
  bottom: 1rem;
  text-align: center;
  background-color: #333;
  color: #fcfdff;
  font-family: 'Alias', sans-serif;
}
</style>
