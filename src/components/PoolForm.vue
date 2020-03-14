<template lang="pug">
  .row
    .col-12.mt-4.a4.card()
      Front(
        :form="page1"
        :isLoggedIn="isLoggedIn"
        @verifyEmail="onVerifyEmail"
        @logOut="onLogOut"
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
import * as firebase from 'firebase/app'

import Front from './Front.vue'
import Back from './Back.vue'
import Menu from './Menu.vue'

import allGames from '../assets/games.json'

import {
  emailRE,
  STORE_EMAIL_KEY,
} from '../constants'

import { vbpStore } from '../helpers'

import * as apiCalls from '../api-calls'

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
  },

  mounted () {
    firebase.auth().onAuthStateChanged((user) => {
      if (user) {
        this.isLoggedIn           = true
        this.page1.userInfo.email = user.email
      } else {
        this.isLoggedIn = false
      }
    })
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

    onVerifyEmail () {
      const { email } = this.page1.userInfo
      if (!emailRE.test(email)) return window.alert('Dit is geen geldig email adres')

      apiCalls.sendSignInLink(email)
        .then(() => {
          vbpStore.save(STORE_EMAIL_KEY, email)
          window.alert(`Er is een email verzonden naar ${email}\nOpen de email om in te loggen`)
        })
        .catch(error => {
          console.error('Something went wrong sending sign in link', error)
          window.alert(`Er is iets misgegaan. Neem AUB even contact met ons op\nerror: ${error.message}`)
        })
    },

    onLogOut () {
      apiCalls.logOut()
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
