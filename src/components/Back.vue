<template lang="pug">
  .container
    Header(
      title="Wedstrijdvoorspellingen"
    )
    .explanation
      h3 Uitleg
      p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
    .warning
      h3 Alle uitslagen, ook de toto, gelden na 90 minuten voetbal!
      span (plus de eventuele blessuretijd)
    .points
      h3 Punten
      p Ruststand goed: 2 pnt, Eindstand goed: 3 pnt, Toto goed: 4 pnt. Totaal aantal doelpunten op één dag goed: 5 pnt.
    .tables
      table.w50.m-right-025
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
          td
            MatchInput(
            )
      table.w50.m-left-025
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
          td
            MatchInput(
            )

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

<style scoped>
.container {
  padding: 0.5rem;
  height: 100%;
  font-family: 'Times New Roman', serif;
}

.explanation {
  margin-top: 1rem;
}

.warning {
  text-align: center;
}

.tables {
  display: flex;
  margin-top: 1rem;
}

.tables th {
  font-weight: normal;
}

.tables input {
  width: 2rem;
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
  position: absolute;
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
