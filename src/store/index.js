import Vue from 'vue'
import Vuex from 'vuex'
import { getField, updateField } from 'vuex-map-fields'

import { getDefaultData } from '../helpers'

import * as firebase from 'firebase/app'

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
  getters: {
    getField,
  },
  mutations: {
    updateField,
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
    setUserPools (state, value) {
      state.user.pools = value
    },
  },
  actions: {
    async createPool ({ commit, state }) {
      const { poolName } = state.form.page1.meta
      if (!poolName) return
      const db       = firebase.firestore()
      const response = await db.collection('pools').add({
        name:   poolName,
        userId: firebase.auth().currentUser.uid,
      })
      console.log('response', response)
      // TODO lock name field unlock form fields
    },
    async fetchUserPools ({ commit, state }) {
      const db       = firebase.firestore()
      const response = await db
        .collection('pools')
        .where('userId', '==', firebase.auth().currentUser.uid)
        .get()
      console.log('response', response)
      response.forEach(doc => console.log(doc.data()))
      commit('setUserPools', response.docs.map(doc => ({ id: doc.id, ...doc.data() })))
    },
  },
})

export default store
