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
    .col-12.mb-4.a4.card()
      Menu(
        @discard="discard"
        @send="send"
        :errors="errors"
      )
</template>

<script>
import * as firebase from 'firebase/app'

import Front from './Front.vue'
import Back from './Back.vue'
import Menu from './Menu.vue'

import allGames from '../assets/games.json'

import { mapState } from 'vuex'

export default {
  name: 'PoolForm',

  components: {
    Front,
    Back,
    Menu,
  },

  data () {
    return this.getDefaultData()
  },

  computed: {
    ...mapState([ 'isLoggedIn' ])
  },

  watch: {
  },

  mounted () {
  },

  methods: {
    send () {
      // Validate logged in
      // Validate form
      console.log('send')
    },

    discard () {
      const shouldDiscard = confirm('Weet je zeker dat je alles wilt wissen?!')
      if (!shouldDiscard) return
      const data  = this.getDefaultData()
      this.errors = []
      this.page1  = data.page1
      this.page2  = data.page2
    },

    getDefaultData () {
      return {
        errors: [],
        page1:  {
          userInfo: {
            name:    '',
            address: '',
            email:   this.page1 ? this.page1.userInfo.email : '', // FIXME Bit hacky
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
  },

}
</script>

<style scoped>
.a4 {
  background: #fcfdff;
  padding-bottom: 3rem;
}

.back {
  margin-top: 2rem;
}

</style>
