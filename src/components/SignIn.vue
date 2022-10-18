<template lang="pug">
.row
  .col-10.mt-4.a4.card.mx-auto
    .row
      .col-12.text-center.mt-3
        h3 Welkom bij de Laurierboom Voetbalpool EK2020
    .row.text-center.mt-5.mb-5
      .col-sm-12.col-lg-8.mx-auto
        div(v-if="!isVerifcationMailSent")
          .d-flex
            h3 Email:
            InfoInput.email-input(
              inputType="email"
              :editable="true"
              v-model="userInfo.email"
            )
          button.btn.btn-outline-primary.mt-4(
            @click="onVerifyEmail"
          ) Verifiëren
        div(v-else)
          h3 Email verzonden, check AUB uw email inbox
</template>

<script>
import InfoInput from './InfoInput'

import {
  emailRE,
  STORE_EMAIL_KEY,
} from '../constants'

import { vbpStore } from '../helpers'

import * as apiCalls from '../api-calls'

import { mapState, mapMutations } from 'vuex'

export default {
  name: 'SignIn',

  components: {
    InfoInput,
  },

  data () {
    return {
      userInfo: {
        email: '',
      },
    }
  },

  mounted () {
  },

  computed: {
    ...mapState({
      isLoggedIn:            state => state.user.isLoggedIn,
      isVerifcationMailSent: state => state.user.isVerifcationMailSent,
    }),
  },

  methods: {
    ...mapMutations([ 'setVerificationMailSent' ]),
    onVerifyEmail () {
      const { email } = this.userInfo
      if (!emailRE.test(email)) return window.alert('Dit is geen geldig email adres')

      apiCalls.sendSignInLink(email)
        .then(() => {
          vbpStore.save(STORE_EMAIL_KEY, email)
          window.alert(`Er is een email verzonden naar ${email}\nOpen de email om in te loggen`)
          this.setVerificationMailSent()
        })
        .catch(error => {
          console.error('Something went wrong sending sign in link', error)
          window.alert(`Er is iets misgegaan. Neem AUB even contact met ons op\nerror: ${error.message}`)
        })
    },

    onLogOut () {
      apiCalls.logOut()
    },
  },

  props: {
    email: String,
  },
}
</script>

<style scoped>
.verify-card {
  min-height: 12rem;
}
</style>
