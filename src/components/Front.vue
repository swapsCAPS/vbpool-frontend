<template lang="pug">
  div
    div
      .row
        .col
          Header(
            title="Inschrijfformulier inleg: € 10,00"
          )
      .row
        .col
          .user-info
            .row.mt-2.mb-2
              .col-12.mw-auto
                InfoInput(
                  title="Poolnaam"
                  :editable="true"
                  v-model.trim="page1.meta.poolName"
                )
      .row
        .col-12
          .instructions
            h3 Instructies
            p Hier onder kun je voorspellingen invoeren voor het EK 2020 van 12 juni 2020 tot 12 juli 2020.
            p Voor elke juiste voorspelling krijg je punten, bij de verschillende onderdelen staat hoeveel.
            p De voorspellingen hoeven niet te kloppen, bij een uitslag kun je bijvoorbeeld 1-0 bij de rust, 0-2 bij de eindstand en een 3 bij de toto invullen. Of je kunt een team dat je uitgeschakeld hebt in een volgende ronde toch weer opnemen.
        .col-12
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
      .row
        .col
          .row
            .col
              Section(
                text="Groepstanden"
                subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
              )
                template(v-slot:content)
                  .row
                    .col-6.col-md-4.col-lg-3.col-xl-2(v-for="group in groups")
                      GroupStage(
                        :group="group"
                        :teams="teams"
                        v-model="page1.groupStances[group.name]"
                      )
          .row
            .col
              Section(
                text="Achtste finales"
                subtext="(5 pnt voor elk genoemd team of 8 pnt als het ook nog op de juiste plaats staat)"
              )
                template(v-slot:content)
                  .row
                    .col-12.col-md-6.col-lg-4.col-xl-3(v-for="game in games.eighth")
                      Game(
                        :game="game"
                        :teams="teams"
                        v-model="page1.finals.eighth[game.nr]"
                      )
          .row
            .col
              Section(
                text="Kwarfinales"
                subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
              )
                template(v-slot:content)
                  .row
                    .col-12.col-md-6.col-lg-4.col-xl-3(v-for="game in games.quarter")
                      Game(
                        :game="game"
                        :teams="teams"
                        v-model="page1.finals.quarter[game.nr]"
                      )
          .row
            .col-12.col-md-12.col-lg-8.col-xl-6
              Section(
                text="Halve finales"
                subtext="(12/16 pnt)"
              )
                template(v-slot:content)
                  .row
                    .col-12.col-md-6(v-for="game in games.half")
                      Game(
                        :game="game"
                        :teams="teams"
                        v-model="page1.finals.half[game.nr]"
                      )
            .col-12.col-md-12.col-lg-4.col-xl-6
              Section(
                text="Finale"
                subtext="20/24 pnt"
              )
                template(v-slot:content)
                  .row
                    .col-12.col-md-6.col-lg-12.col-xl-6(v-for="game in games.final")
                      Game(
                        :game="game"
                        :teams="teams"
                        v-model="page1.finals.final"
                      )
          .row
            .col-12.col-md-6.col-lg-3
              Section(
                text="Eindstand"
                subtext=""
              )
                template(v-slot:content)
                  GameSelector(
                    name="1e: "
                    :teams="teams"
                    v-model="page1.endStance[0]"
                  ).vbp-border
                  GameSelector(
                    name="2e: "
                    :teams="teams"
                    v-model="page1.endStance[1]"
                  ).input.vbp-border.vbp-no-top-border
            .col-12.col-md-6.col-lg-3
              Section(
                text="Topscorer & aantal goals"
                subtext=""
              )
                template(v-slot:content)
                  MiscInput(
                    text="Speler (30p)"
                    v-model.trim="page1.topScorer.player"
                  ).vbp-border
                  MiscInput(
                    text="Aantal goals (12p)"
                    inputType="number"
                    v-model.number="page1.topScorer.goals"
                  ).input.vbp-border.vbp-no-top-border
            .col-12.col-md-6
              Section(
                text="Overigen"
                subtext=""
              )
                template(v-slot:content)
                  MiscInput(
                    text="aantal gele kaarten (±4, 16p):"
                    inputType="number"
                    v-model.number="page1.misc.yellowCards"
                  ).vbp-border
                  MiscInput(
                    text="aantal rode kaarten (±2, 16p):"
                    inputType="number"
                    v-model.number="page1.misc.redCards"
                  ).input.vbp-border.vbp-no-top-border
                  MiscInput(
                    text="aantal penalties in speeltijd (±1, 16p):"
                    inputType="number"
                    v-model.number="page1.misc.penalties"
                  ).input.vbp-border.vbp-no-top-border
                  MiscInput(
                    text="aantal gelijke spelen (±3, 16p):"
                    inputType="number"
                    v-model.number="page1.misc.draws"
                  ).input.vbp-border.vbp-no-top-border
                  MiscInput(
                    text="aantal doelpunten in toernooi (±5, 16p):"
                    inputType="number"
                    v-model.number="page1.misc.totalGoals"
                  ).input.vbp-border.vbp-no-top-border
          Disabler(
            :isDisabled="!user.isLoggedIn || !page1.meta.poolName"
            message="Vul eerst een naam in voor deze pool en klik op opslaan"
          )
</template>

<script>
import InfoInput from './InfoInput'
import Header from './Header'
import Section from './Section'
import GroupStage from './GroupStage'
import Game from './Game'
import GameSelector from './GameSelector'
import MiscInput from './MiscInput'
import Disabler from './Disabler'

import groups from '../assets/teams.json'
import allGames from '../assets/games.json'

import { mapState, mapMutations } from 'vuex'

export default {
  name: 'Front',

  components: {
    Game,
    GameSelector,
    GroupStage,
    Header,
    InfoInput,
    MiscInput,
    Disabler,
    Section,
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
    ...mapState([ 'user', 'page1' ]),
    games () {
      return this.allGames.games.reduce((acc, game) => {
        if (!game.type) return acc
        acc[game.type].push(game)
        return acc
      }, { eighth: [], quarter: [], half: [], final: [] })
    },
    teams () {
      return groups
        .map(({ teams }) => teams)
        .reduce((acc, teams) => acc.concat(teams), [])
        .map(({ team }) => team)
        .sort()
    },
  },
}
</script>

<style scoped>
.user-info {
  border: 1px solid #ccc;
  padding: 0rem 0.5rem;
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

.email-input-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.email-input-wrapper button {
  min-width: 6rem;
}

.email-input {
  overflow: hidden;
}

</style>
