<template lang="pug">
  div
    .flip-container(
    )
      .a4(
        ref="a4"
        :class="{ rotate: rotated }"
      )
        .a4-front(
          :class="{ 'fade-out': !!rotated, 'fade-in': !rotated, 'display-none': !!isDisabled }"
        )
          Front()
          span.zoz(
            v-on:click="doFlip"
          ) z.o.z
        .a4-back(
          :class="{ 'fade-out': !rotated, 'fade-in': !!rotated, 'display-none': !isDisabled }"
        )
          Back()
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
      rotated:    true,
      isDisabled: true,
    }
  },

  computed: {
  },

  props: {},

  created () {
  },

  mounted () {
    this.handleResize()
  },

  destroyed () {
  },

  methods: {
    doFlip () {
      this.rotated = !this.rotated

      // FIXME Meh can we do this nicer using css
      setTimeout(() => {
        this.isDisabled = !!this.rotated
      }, 600)
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
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  min-width: 740px;
  max-width: 1000px;
  height: 1414px;
  margin-left: auto;
  margin-right: auto;
  transition: transform 0.8s;
  transform-style: preserve-3d;
  transform-origin: center right;
  transition: 2s;
  transform: scale(1);
}

.rotate {
  transform: translateX(-100%) rotateY(-180deg);
}

.fade-out {
  -webkit-transition: opacity 0.5s ease-in-out;
  -moz-transition: opacity 0.5s ease-in-out;
  -ms-transition: opacity 0.5s ease-in-out;
  -o-transition: opacity 0.5s ease-in-out;
   opacity: 0;
}

.fade-in {
  -webkit-transition: opacity 1.5s ease-in-out;
  -moz-transition: opacity 1.5s ease-in-out;
  -ms-transition: opacity 1.5s ease-in-out;
  -o-transition: opacity 1.5s ease-in-out;
   opacity: 1;
}

.display-none {
  display: none;
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
