<template lang="pug">
  .row
    .col-12.mt-4.a4.card()
      Front(
        :form="page1"
      )
    .col-12.mb-4.a4.card.back()
      Back()
</template>

<script>

import Front from './Front.vue'
import Back from './Back.vue'

export default {
  name: 'PoolForm',

  data () {
    return {
      // TODO test if too far nested?
      // TODO vuex?
      page1: {
        userInfo: {
          name:    '',
          address: '',
          email:   '',
          phone:   '',
          city:    '',
        },
        groupStances: {
          A: {},
          B: {},
          C: {},
          D: {},
          E: {},
          F: {},
        },
        finals: {
          eighth: [

          ],
          quarter: [

          ],
          half:  [],
          final: [],
        },
        endStance: {
          first:  '',
          second: '',
        },
        topScorer: {
          player: '',
          goals:  null,
        },
        misc: {
          yellowCards: null,
          redCards:    null,
          penalties:   null,
          draws:       null,
          totalGoals:  null,
        },
      },
      page2: {
        // TODO obj w/ nr, rest, end, toto
        games: [],
      },
    }
  },

  components: {
    Front,
    Back,
  },

}
</script>

<style scoped>
.a4 {
  background: #fcfdff;
  padding-bottom: 3rem;
  overflow: hidden;
}

.back {
  margin-top: 2rem;
}

</style>
