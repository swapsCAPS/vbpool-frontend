<template lang="pug">
  div
    .flip-container(
    )
      .a4(
        ref="a4"
        v-bind:class="{ rotate: rotated }"
        v-bind:style="{ height: `${a4Height}px` }"
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
      rotated:  true,
      a4Height: 1414,
    }
  },

  computed: {
  },

  props: {},

  created () {
    window.addEventListener('resize', this.handleResize)
  },

  mounted () {
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
      if (!this.$refs.a4) {
        this.a4Height = 1414
        return
      }
      console.log('this.$refs.a4.clientWidth', this.$refs.a4.clientWidth, this.$refs.a4.clientHeight)
      this.a4Height = this.$refs.a4.clientWidth * 1.414
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
  margin: 3rem 1rem;
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
  max-width: 1000px;
  height: 1414px;
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
