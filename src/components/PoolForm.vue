<template lang="pug">
  .row
    .col-12.mt-4.a4.card()
      Front(
        :form="page1"
        :isLoggedIn="isLoggedIn"
        @validateEmail="onValidateEmail"
      )
    .col-12.mb-4.a4.card.back()
      Back(
        :form="page2"
        :isLoggedIn="isLoggedIn"
      )
    .col-12.mb-4.a4.card()
      Menu(
        @discard="discard"
        @send="send"
        :errors="errors"
        :isLoggedIn="isLoggedIn"
      )
</template>

<script>

import Front from './Front.vue'
import Back from './Back.vue'
import Menu from './Menu.vue'

import allGames from '../assets/games.json'

import { emailRE } from '../constants'

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
    send () {
      // Validate logged in
      // Validate form
      console.log('send')
    },

    discard () {
      const shouldDiscard = confirm('Weet je zeker dat je alles wilt wissen?!')
      if (!shouldDiscard) return
      window.localStorage.clear()
      const data  = this.getDefaultData()
      this.errors = []
      this.page1  = data.page1
      this.page2  = data.page2
    },

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

    onValidateEmail () {
      const { email } = this.page1.userInfo
      if (!emailRE.test(email)) return window.alert('Dit is geen geldig email adres')
      console.log('value', email)
    },

    getDefaultData () {
      return {
        errors: [],
        page1:  {
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
        isLoggedIn: false,
      }
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
