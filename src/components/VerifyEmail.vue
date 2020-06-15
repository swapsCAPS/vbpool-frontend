<template lang="pug">
  .row
    .col-12.mt-4.a4.card.verify-card()
      .mt-5.mb-5
        .text-center(v-if="success")
          h1 Je bent ingelogd. Veel plezier met invullen!
        div(v-else-if="error")
          h1.text-center Er is iets misgegaan tijdens het inloggen...
          h3.mt-3 Neem AUB even contact met ons op
          h5.mt-4
            span Melding:
            span {{ ' ' }}
            span {{ error }}
        .text-center(v-else)
          h1 Bezig met inloggen...
</template>

<script>
import * as firebase from 'firebase/app'

import { mapMutations } from 'vuex'

import { sleep, fbAuthObservablePromiseWrapper, vbpStore } from '../helpers'
import { STORE_EMAIL_KEY } from '../constants'

export default {
  name: 'VerifyEmail',

  data () {
    return {
      loggedIn: false,
      success:  false,
      error:    '',
    }
  },

  methods: {
    ...mapMutations([ 'setLoggedIn' ]),
  },

  async mounted () {
    if (firebase.auth().isSignInWithEmailLink(window.location.href)) {
      let storedEmail = vbpStore.load(STORE_EMAIL_KEY)

      if (!storedEmail) {
        // User opened the link on a different device. To prevent session fixation
        // attacks, ask the user to provide the associated email again. For example:
        storedEmail = window.prompt('We konden je inlog-actie niet vinden voor deze browser\nVul AUB nogmaals je email adres in')
      }

      let user
      try {
        await firebase.auth().signInWithEmailLink(storedEmail, window.location.href)
        user = await fbAuthObservablePromiseWrapper()
      } catch (error) {
        // Common errors could be invalid email and invalid or expired OTPs.
        this.error = error.message
        return console.log('Something went wrong verifying email', error)
      } finally {
        // Clear email from storage.
        vbpStore.delete(STORE_EMAIL_KEY)
      }

      this.success = true

      this.$store.commit('setLoggedIn', true)
      this.$store.commit('setUser', user)

      await sleep(1000)

      this.$router.push({ name: 'form' })
    }
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
