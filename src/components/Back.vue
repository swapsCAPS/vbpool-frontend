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
    .table
      table
        tr
          th Datum
          th tijd
          th nr
          th Wedstrijd
          th rust
          th eind
          th toto

    .footer
      h2 UITERLIJK INLEVEREN OP MAANDAG 8 JUNI 2020
</template>

<script>

import Header from './Header'

export default {
  name: 'Back',

  components: {
    Header,
  },

  props: {
    disableInputs: Boolean,
  },

  data: function () {
    return { }
  },

  computed: {
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

.table {
  margin-top: 1rem;
}

.table th {
  font-weight: normal;
}

.table table, td {
  border: 1px solid black;
  border-collapse: collapse;
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
