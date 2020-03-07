<template lang="pug">
  .container
    Header(
      title="Inschrijfformulier   inleg: € 10,00"
    )
    .user-info
      .left
        InfoInput(
          title="Naam"
          fieldName="name"
        )
        InfoInput(
          title="Adres"
          fieldName="address"
        )
        InfoInput(
          title="Email"
          fieldName="email"
        )
      .right
        InfoInput(
          title="Telefoon"
          fieldName="phone"
        )
        InfoInput(
          title="Plaats"
          fieldName="city"
        )
        InfoInput(
          title="Betaald"
          fieldName="paid"
        )
    .instructions
      h3 Instructies
      p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
    .prizes
      h3 Prijzen
      ul
        li Per dag zijn de volgende geldprijzen te verdienen:
          ul
            li Degene die op één dag de meeste punten heeft verzameld, krijgt daarvoor € 2,50
            li Degene die na een dag in de totaalstand bovenaan staat, krijgt daarvoor € 1,00
            li Degene die na een dag in de totaalstand onderaan staat, krijgt daarvoor als troost € 0,10
            li
              p De punten voor de finalerondes tellen mee voor de dagprijs op de dag dat de teams bekend zijn
              p De punten voor de eindstand, topscorers en aantallen tellen op de dag van de finale mee voor de dagprijs
        li
          span.bold {{ `Aan het eind van het toernooi ` }}
          span zijn de volgende geldprijzen te verdienen:
          ul
            li De rode lantaarn ontvangt als troostprijs € 10,00
            li
              p De hoogste deelnemers in de totaalstand krijgen de volgende prijzen:
              p 1e pl: 50%, 2de pl: 30%, 3e pl: 15%, 4e pl: 5% van de totale inleg (minus de dagprijzen en de rode lantaarn)
        li Bij een gelijk aantal punten wordt de betreffende prijs verdeeld
    .groups
      Section(
        text="Groepstanden"
        subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
      )
        template(v-slot:content)
          .groups-container
            div(v-for="group in groups")
              GroupStage(
                :group="group"
              )
    .eighth
      Section(
        text="Achtste finales"
        subtext="(5 pnt voor elk genoemd team of 8 pnt als het ook nog op de juiste plaats staat)"
      )
        template(v-slot:content)
          .flex-wrap
            .game.w25.m-top-05.m-bottom-025(v-for="game in games.eighth")
              Game(:game="game")
    .quarter
      Section(
        text="Kwarfinales"
        subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
      )
        template(v-slot:content)
          .flex-wrap
            .game.w25.m-top-05.m-bottom-025(v-for="game in games.quarter")
              Game(:game="game")
    .finals
      .half
        Section(
          text="Halve finales"
          subtext="(12/16 pnt)"
        )
          template(v-slot:content)
            .flex-wrap
              .game.w50.m-top-05.m-bottom-025(v-for="game in games.half")
                Game(:game="game")
      .final
        Section(
          text="Finale"
          subtext="20/24 pnt"
        )
          template(v-slot:content)
            .flex-wrap
              .game.w100.m-top-05.m-bottom-025(v-for="game in games.final")
                Game(:game="game")
    .ending
      .end-stance.w25
        Section(
          text="Eindstand"
          subtext=""
        )
          template(v-slot:content)
            .inputs.m-bottom-025
              .game-input-wrapper
                span 1e:
                input(type="text")
              .game-input-wrapper.no-top-border
                span 2e:
                input(type="text")
      .top-scorer-goals.w25.m-left-05
        Section(
          text="Topscorer & aantal goals"
          subtext=""
        )
          template(v-slot:content)
            .game-input-wrapper
              span 1:
              input(type="text")
            .game-input-wrapper
              span Aantal goals:
              input(type="text")
      .misc.w50.m-left-05
        Section(
          text="Overigen"
          subtext=""
        )
          template(v-slot:content)
            .game-input-wrapper
              span aantal gele kaarten (±4, 16p):
              input(type="text")
            .game-input-wrapper
              span aantal rode kaarten (±2, 16p):
              input(type="text")
            .game-input-wrapper
              span aantal penalties in speeltijd (±1, 16p):
              input(type="text")
            .game-input-wrapper
              span aantal gelijke spelen (±3, 16p):
              input(type="text")
            .game-input-wrapper
              span aantal doelpunten in toernooi (±5, 16p):
              input(type="text")
</template>

<script>
import InfoInput from './InfoInput'
import Header from './Header'
import Section from './Section'
import GroupStage from './GroupStage'
import Game from './Game'

import groups from '../assets/teams.json'
import allGames from '../assets/games.json'

export default {
  name: 'Front',

  components: {
    InfoInput,
    Header,
    Section,
    GroupStage,
    Game,
  },

  props: {
  },

  data: function () {
    return {
      groups,
      allGames,
    }
  },

  computed: {
    games () {
      return this.allGames.games.reduce((acc, game) => {
        if (!game.type) return acc
        acc[game.type].push(game)
        return acc
      }, { eighth: [], quarter: [], half: [], final: [] })
    },
  },

  mounted () {
    console.log(this.groups)
  },

  methods: { },
}
</script>

<style scoped>
.container {
  padding: 0.5rem;
  font-family: 'Times New Roman', serif;
}

.user-info {
  border: 1px solid #ccc;
  overflow: hidden;
}

.user-info .left {
  float: left;
  width: 55%;
}

.user-info .right {
  float: right;
  width: 45%;
}

.prizes ul {
  list-style: none;
  margin-top: 0;
  margin-left: 0;
  padding-left: 1rem;
  position: relative;
}

.prizes ul li:before {
  content: "-";
  position: absolute;
  left: 0;
}

.groups-container {
  display: flex;
  justify-content: space-between;
}

.flex-wrap {
  display: flex;
  flex-wrap: wrap;
}

.game {
  display: flex;
  overflow: hidden;
}

.finals {
  display: flex;
}

.finals .half {
  width: 50%;
}

.finals .final {
  margin-left: 0.5rem;
  width: 50%;
}

.ending {
  display: flex;
}

.ending .misc .game-input-wrapper input, span {
  flex-grow: 1;
}

</style>
