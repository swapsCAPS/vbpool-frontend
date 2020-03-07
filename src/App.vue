<template lang="pug">
  #app
    PoolForm
</template>

<script>
import PoolForm from './components/PoolForm.vue'

export default {
  name:       'App',
  components: {

    PoolForm,
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css?family=Gochi+Hand&display=swap');
@import url('https://fonts.googleapis.com/css?family=Cormorant+Garamond:300,300i,400&display=swap');
h1, h2, h3, h4, h5, p {
  margin: 0;
}

h3 {
  margin-bottom: 0.25rem;
}

p {
  font-size: 12pt;
}

body {
  margin: 0 !important;
  padding: 0 !important;
  background-color: #fff8f2;
  color: #666;
  cursor: default;
}

input {
  padding: 0;
  font-family: 'Gochi Hand', cursive;
  border: 0;
}

/* Disable number spinners */
/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}

#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  position: relative;
}

.game-input-wrapper {
  position: relative;
  border: 1px solid #ccc;
  display: flex;
}

.game-input-wrapper-bottom {
  border-top: none;
}

.game-input-wrapper span {
  flex-grow: 1;
}

.game-input-wrapper input {
  border: 0;
  flex-grow: 9;
}

.no-top-border {
  border-top: none !important;
}

.w25 {
  width: 25%;
}

.w50 {
  width: 50%;
}

.w100 {
  width: 100%;
}

.m-top-025 {
  margin-top: 0.5rem;
}

.m-top-05 {
  margin-top: 0.5rem;
}

.m-left-025 {
  margin-left: 0.25rem;
}

.m-left-05 {
  margin-left: 0.5rem;
}

.m-bottom-025 {
  margin-bottom: 0.25rem;
}

.m-bottom-05 {
  margin-bottom: 0.5rem;
}

.m-right-025 {
  margin-right: 0.25rem;
}

.m-right-05 {
  margin-right: 0.5rem;
}

.bold {
  font-weight: bold;
}

@media screen and (max-width: 800px) {
  body {
    font-size: 10pt;
  }

  h1 {
    font-size: 14pt;
  }

  h2 {
    font-size: 12pt;
  }

  h3 {
    font-size: 10pt;
  }

  h4 {
    font-size: 10pt;
  }

  h5 {
    font-size: 10pt;
  }

  p {
    line-height: 1.0rem;
  }

  td {
    line-height: 1.2rem;
  }
}

</style>
