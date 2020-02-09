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
          :isDisabled="disableInputs"
        )
        InfoInput(
          title="Adres"
          fieldName="address"
          :isDisabled="disableInputs"
        )
        InfoInput(
          title="Email"
          fieldName="email"
          :isDisabled="disableInputs"
        )
      .right
        InfoInput(
          title="Telefoon"
          fieldName="phone"
          :isDisabled="disableInputs"
        )
        InfoInput(
          title="Plaats"
          fieldName="city"
          :isDisabled="disableInputs"
        )
        InfoInput(
          title="Betaald"
          fieldName="paid"
          :isDisabled="disableInputs"
        )
    .instructions
      h3 Instructies
      p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
    .prizes
      h3 Prijzen
      ul
        li Per dag zijn de volgende geldprijzen te verdienen:
          ul
            li bla
            li bla
            li bla
            li bla
        li Aan het eind van het toernooi
          ul
            li bla
            li bla
        li Bij een gelijk aantal punten
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
            .game(v-for="game in games.eighth")
              span.game-nr {{ game.nr }}
              div.inputs
                .game-input-wrapper
                  span {{ game.naam1 }}
                  input(type="text")
                .game-input-wrapper
                  span {{ game.naam2 }}
                  input(type="text")
    .quarter
      Section(
        text="Kwarfinales"
        subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
      )
        template(v-slot:content)
          .flex-wrap
            .game(v-for="game in games.quarter")
              span.game-nr {{ game.nr }}
              div.inputs
                .game-input-wrapper
                  span {{ game.naam1 }}
                  input(type="text")
                .game-input-wrapper
                  span {{ game.naam2 }}
                  input(type="text")
    .finals
      .half
        Section(
          text="Halve finales"
          subtext="(12/16 pnt)"
        )
          template(v-slot:content)
            .flex-wrap
              .game.half(v-for="game in games.half")
                span.game-nr {{ game.nr }}
                div.inputs
                  .game-input-wrapper
                    span {{ game.naam1 }}
                    input(type="text")
                  .game-input-wrapper
                    span {{ game.naam2 }}
                    input(type="text")
      .final
        Section(
          text="Finale"
          subtext="20/24 pnt"
        )
          template(v-slot:content)
            .flex-wrap
              .game.half(v-for="game in games.final")
                span.game-nr {{ game.nr }}
                div.inputs
                  .game-input-wrapper
                    span {{ game.naam1 }}
                    input(type="text")
                  .game-input-wrapper
                    span {{ game.naam2 }}
                    input(type="text")
    .ending
      .stance
        Section(
          text="Eindstand"
          subtext=""
        )
          template(v-slot:content)
            p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
      .top-score
        Section(
          text="Topscorer & aantal goals"
          subtext=""
        )
          template(v-slot:content)
            p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
      .misc
        Section(
          text="Overigen"
          subtext=""
        )
          template(v-slot:content)
            p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
</template>

<script>
import InfoInput from './InfoInput'
import Header from './Header'
import Section from './Section'
import GroupStage from './GroupStage'

import groups from '../assets/teams.json'
import allGames from '../assets/games.json'

export default {
  name: 'Front',

  components: {
    InfoInput,
    Header,
    Section,
    GroupStage,
  },

  props: {
    disableInputs: Boolean,
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
  width: 25%;
  display: flex;
  overflow: hidden;
}

.game .inputs {
  margin-left: 0.5rem;
}

.game-input-wrapper {
  position: relative;
  border: 1px solid #ccc;
  display: flex;
}

.game-input-wrapper span {
  flex-grow: 1;
}

.game-input-wrapper input {
  border: 0;
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

.ending .stance, .top-score {
  width: 25%;
}

.ending .stance {
  margin-right: 0.25rem;
}

.ending .misc {
  margin-left: 0.5rem;
  width: 50%;
}

</style>
