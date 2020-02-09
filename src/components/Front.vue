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
          p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
    .eighth
      Section(
        text="Achtste finales"
        subtext="(5 pnt voor elk genoemd team of 8 pnt als het ook nog op de juiste plaats staat)"
      )
        template(v-slot:content)
          p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
    .quarter
      Section(
        text="Kwarfinales"
        subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
      )
        template(v-slot:content)
          p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
    .finals
      .half
        Section(
          text="Halve finales"
          subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
        )
          template(v-slot:content)
            p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
      .final
        Section(
          text="Finale"
          subtext="Vul in: 1 t/m 4 (5 pnt per correcte invoer)"
        )
          template(v-slot:content)
            p Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque congue, ipsum in dignissim commodo, urna ante varius tortor, euismod tempus leo ante vitae massa. Aliquam tincidunt neque et sem viverra iaculis. Quisque suscipit risus a aliquam ornare. Cras iaculis neque sed nisl congue lobortis. Aenean sed leo feugiat, gravida elit vitae, lacinia diam. Proin malesuada nunc ante, quis mattis ante auctor vitae. Vivamus efficitur massa nunc, eu condimentum lacus accumsan eget. Phasellus felis nunc, elementum ac vehicula efficitur, egestas eu elit.
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

export default {
  name: 'Front',

  components: {
    InfoInput,
    Header,
    Section,
  },

  props: {
    disableInputs: Boolean,
  },

  data: function () {
    return { }
  },

  computed: {
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
