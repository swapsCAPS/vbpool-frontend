<template lang="pug">
div
  .row
    .col-12
      HeaderComponent(
        subtext="Wedstrijdvoorspellingen"
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
    .col-12.position-relative
      .row.tables
        .col-12.col-xl-6
          TableComponent(
            :games="games.left"
            :form="page2"
          )
        .col-12.col-xl-6
          TableComponent(
            :games="games.right"
            :form="page2"
          )
        Disabler(
          :isDisabled="!user.isLoggedIn || !$route.params.poolId"
          message="Vul eerst een naam in voor deze pool en klik op \"Aanmaken\""
        )
  .row
    .col-12
      .footer
        h2 UITERLIJK INLEVEREN OP MAANDAG 8 JUNI 2020
</template>

<script>
import HeaderComponent from './HeaderComponent'
import MatchInput from './MatchInput'
import TableComponent from './TableComponent'
import Disabler from './Disabler'

import { mapState } from 'vuex'

import allGames from '../assets/games.json'

export default {
  name: 'BackComponent',

  components: {
    HeaderComponent,
    MatchInput,
    TableComponent,
    Disabler,
  },

  data: function () {
    return {
      allGames,
    }
  },

  computed: {
    ...mapState([ 'user', 'form', 'route' ]),
    ...mapState({
      page2: state => state.form.page2,
    }),
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
p {
  line-height: 1.25;
  margin-bottom: 0;
}

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
