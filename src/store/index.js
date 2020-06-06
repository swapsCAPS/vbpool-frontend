import Vue from 'vue'
import Vuex from 'vuex'

import { getDefaultData } from '../helpers'

Vue.use(Vuex)

const { page1, page2 } = getDefaultData()

const store = new Vuex.Store({
  state: {
    user: {
      isLoggedIn:            false,
      email:                 '',
      isVerifcationMailSent: false,
      pools:                 [],
    },
    form: {
      type: 'new',
      page1,
      page2,
    },
  },
  mutations: {
    discard (state) {
      const shouldDiscard = confirm('Weet je zeker dat je alles wilt wissen?!')
      if (!shouldDiscard) return
      const { page1, page2 } = getDefaultData()
      state.form.page1       = page1
      state.form.page2       = page2
    },
    setLoggedIn (state, value) {
      state.user.isLoggedIn = value
    },
    setEmail (state, value) {
      state.user.email = value
    },
    setVerificationMailSent (state) {
      state.user.isVerifcationMailSent = true
    },
  },
  actions: {
    sendPool ({ commit, state }) {
      console.log(state.page1)
      console.log(state.page2)
    },
  },
})

export default store
