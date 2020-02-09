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
    .prizes
    .groups
    .eighth
    .quarter
    .half
    .end
</template>

<script>
import InfoInput from './InfoInput'
import Header from './Header'

export default {
  name: 'Front',

  components: {
    InfoInput,
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

  methods: { },
}
</script>

<style scoped>
h1, h2, h3 {
  margin: 0;
}
.container {
  padding: 0.5rem;
}

.head {
  font-family: 'Cormorant Garamond', serif;
  font-weight: 300;
}

.head h1 {
  text-align: center;
  margin: 0.2rem 0;
}

.head h2 {
  text-align: center;
  background-color: #333;
  color: green;
  font-style: italic;
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
</style>
