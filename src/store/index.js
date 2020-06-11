import Vue from 'vue'
import _ from 'lodash'
import Vuex from 'vuex'
import { getField, updateField } from 'vuex-map-fields'

import { getDefaultData } from '../helpers'

import * as firebase from 'firebase/app'

import router from '../router'

Vue.use(Vuex)

const { page1, page2 } = getDefaultData()

const store = new Vuex.Store({
  state: {
    user: {
      isLoggedIn:            false,
      role:                  'user',
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
    setRole (state, value) {
      state.user.role = value
    },
    setVerificationMailSent (state) {
      state.user.isVerifcationMailSent = true
    },
    setUserPools (state, value) {
      state.user.pools = value
    },
    upsertFormPages (state, value) {
      state.form = _.merge(state.form, value)
    },
  },
  actions: {
    async createPool ({ commit, dispatch, state }) {
      const { poolName } = state.form.page1.meta

      if (!poolName) return

      const db = firebase.firestore()

      const res = await db.collection('pools').add({
        userId: firebase.auth().currentUser.uid,
        form:   state.form,
      })

      console.log('res', res)
      router.push({ name: 'edit-form', params: { poolId: res.id } })
    },

    async fetchAndSetPool ({ commit }, id) {
      if (!id) throw new Error('fetchAndSetPool called w/o an id')

      const db = firebase.firestore()

      commit('discard')

      const doc = await db.collection('pools').doc(id).get()

      if (!doc.exists) {
        throw new Error(`No pool with id: ${id}`)
      }

      commit('upsertFormPages', doc.data().form)
    },

    async fetchUserPools ({ commit, state }) {
      const db = firebase.firestore()

      const response = await db
        .collection('pools')
        .where('userId', '==', firebase.auth().currentUser.uid)
        .get()

      commit('setUserPools', response.docs.map(doc => ({ id: doc.id, ...doc.data() })))
    },
  },
})

export default store
