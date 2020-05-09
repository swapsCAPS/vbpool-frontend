<template lang="pug">
  div
    .row
      .col-12
        Header(
          title="Wedstrijdvoorspellingen"
        )
    .row
      .col-12
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
      .col-12
        .row.tables
          .col-12.col-xl-6
            Table(
              :games="games.left"
              :form="form"
            )
          .col-12.col-xl-6
            Table(
              :games="games.right"
              :form="form"
            )
          Disabler(:isDisabled="!user.isLoggedIn")
    .row
      .col-12
        .footer
          h2 UITERLIJK INLEVEREN OP MAANDAG 8 JUNI 2020
</template>

<script>
import Header from './Header'
import MatchInput from './MatchInput'
import Table from './Table'
import Disabler from './Disabler'

import { mapState } from 'vuex'

import allGames from '../assets/games.json'

export default {
  name: 'Back',

  components: {
    Header,
    MatchInput,
    Table,
    Disabler,
  },

  props: {
    form:       Object,
    isLoggedIn: Boolean,
  },

  data: function () {
    return {
      allGames,
    }
  },

  computed: {
    ...mapState([ 'user' ]),
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

<style scoped>
.container {
  font-family: 'Times New Roman', serif;
}

.explanation {
  margin-top: 1rem;
}

.warning {
  text-align: center;
}

.footer {
  margin-top: 2rem;
  bottom: 2rem;
  left: 0.5rem;
  right: 0.5rem;
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
