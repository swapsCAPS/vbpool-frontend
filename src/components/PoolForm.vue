<template lang="pug">
  .row
    .col-12.mt-4.a4.card()
      Front(
        :form="page1"
      )
    .col-12.mb-4.a4.card.back()
      Back(
        :form="page2"
      )
</template>

<script>

import Front from './Front.vue'
import Back from './Back.vue'

import allGames from '../assets/games.json'

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
          eighth: {
            37: [ '', '' ],
            38: [ '', '' ],
            39: [ '', '' ],
            40: [ '', '' ],
            41: [ '', '' ],
            42: [ '', '' ],
            43: [ '', '' ],
            44: [ '', '' ],
          },
          quarter: {
            45: [ '', '' ],
            46: [ '', '' ],
            47: [ '', '' ],
            48: [ '', '' ],
          },
          half: {
            49: [ '', '' ],
            50: [ '', '' ],
          },
          final: [ '', '' ],
        },
        endStance: [ '', '' ],
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
      page2: allGames.games.reduce((acc, g) => {
        acc[g.nr] = {
          half: [ '', '' ],
          end:  [ '', '' ],
          toto: null,
        }
        return acc
      }, {}),
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
