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

  components: {
    Front,
    Back,
  },

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

  watch: {
    page1: {
      deep: true,
      handler () {
        this.store('page1', this.page1)
      },
    },
    page2: {
      deep: true,
      handler () {
        this.store('page2', this.page2)
      },
    },
  },

  mounted () {
    const page1 = this.get('page1')
    const page2 = this.get('page2')
    if (page1) this.page1 = page1
    if (page2) this.page2 = page2
  },

  methods: {
    store (key, data) {
      window.localStorage.setItem(key, JSON.stringify(data))
    },
    get (key) {
      let data
      const stored = window.localStorage.getItem(key)
      if (!stored) return console.log(`No data for ${key}`)
      try {
        data = JSON.parse(stored)
      } catch (error) {
        return console.error(`Could not parse ${key} from store`)
      }
      return data
    },
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
