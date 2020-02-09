<template lang="pug">
  div
    .flip-container(
    )
      .a4(
        v-bind:class="{ rotate: rotated }"
      )
        .a4-front
          Front(
            :disableInputs="!!rotated"
          )
          span.zoz(
            v-on:click="doFlip"
          ) z.o.z
        .a4-back
          Back(
            :disableInputs="!rotated"
          )
          span.zoz(
            v-on:click="doFlip"
          ) z.o.z
</template>

<script>

import Front from './Front.vue'
import Back from './Back.vue'

export default {
  name: 'PoolForm',

  components: {
    Front,
    Back,
  },

  data: function () {
    return {
      rotated: false,
    }
  },

  computed: {
  },

  props: {},

  created () {
    window.addEventListener('resize', this.handleResize)
    this.handleResize()
  },

  destroyed () {
    window.removeEventListener('resize', this.handleResize)
  },

  methods: {
    doFlip () {
      this.rotated = !this.rotated
    },
    handleResize () {
      this.window = {
        width:  window.innerWidth,
        height: window.innerHeight,
      }
    },
  },
}
</script>

<style scoped>
h1 {
  margin: 0;
}
.flip-container {
  position: relative;
  background-color: transparent;
  padding-top: 141.4%;/* 141.4%; */
  margin: 1rem 1rem;
  perspective: 3000px;
}

.a4 {
  position: absolute;
  margin-left: auto;
  margin-right: auto;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  min-width: 740px;
  max-width: 1200px;
  transition: transform 0.8s;
  transform-style: preserve-3d;
  transform-origin: center right;
  transition: 2s;
  transform: scale(1);
}

.rotate {
  transform: translateX(-100%) rotateY(-180deg);
}

.a4-front, .a4-back {
  background: #fcfdff;
  border: 2px solid #ccc;
  position: absolute;
  width: 100%;
  height: 100%;
  -webkit-backface-visibility: hidden; /* Safari */
  backface-visibility: hidden;
}

.a4-back {
  transform: rotateY(180deg);
}

.zoz {
  position: absolute;
  bottom: 0.2rem;
  right: 0.2rem;
  cursor: pointer;
}

</style>
