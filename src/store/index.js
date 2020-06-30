import Vue from 'vue'
import _ from 'lodash'
import Vuex from 'vuex'
import { getField, updateField } from 'vuex-map-fields'

import { getDefaultData } from '../helpers'

import firebase from 'firebase/app'

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
      page1,
      page2,
    },
    isSaving: false,
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
    setUser (state, user) {
      state.user = { ...state.user, ..._.pick(user, [ 'email', 'role' ]) }
    },
    resetUser (state) {
      state.user = { ...state.user, role: 'user', email: '' }
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
    setSaving (state, value) {
      state.isSaving = value
    },
  },
  actions: {
    async createPool ({ state }) {
      if (!state.form.page1.meta.poolName) return

      const db = firebase.firestore()

      const list = await db
        .collection('pools')
        .where('userId', '==', firebase.auth().currentUser.uid)
        .get()
      if (list.docs.length >= 10) return alert('Je kunt niet meer dan 10 pools aanmaken')

      const userId = firebase.auth().currentUser.uid

      const pool = db.collection('pools').doc()
      const form = db.collection('pools').doc(pool.id).collection('forms').doc('form')

      await pool.set({
        userId,
        meta: {
          poolName: state.form.page1.meta.poolName,
        },
        flags: {
          isPaid:     false,
          isComplete: false,
        },
      })

      await form.set(state.form)

      router.push({ name: 'edit-form', params: { poolId: pool.id } })
    },

    async readPool ({ commit }, id) {
      if (!id) throw new Error('readPool called w/o an id')

      const db = firebase.firestore()

      commit('discard')

      // TODO might not need pool here... Saves a db read.
      const [ pool, form ] = await Promise.all([
        db.collection('pools').doc(id).get(),
        db.collection('pools').doc(id).collection('forms').doc('form').get(),
      ])

      if (!pool.exists) {
        throw new Error(`No pool with id: ${id}`)
      }

      commit('upsertFormPages', form.data())
    },

    async updatePool ({ state, commit }) {
      const { poolId } = state.route.params
      if (!poolId) return

      if (state.isSaving) return // Maybe we should cancel the current and invoke the next save...

      commit('setSaving', true)

      const db = firebase.firestore()

      // TODO validation
      await db.collection('pools').doc(poolId).collection('forms').doc('form').set(state.form)

      setTimeout(() => {
        commit('setSaving', false)
      }, 1500)
    },

    async deletePool ({ state, commit }) {
      const { poolId } = state.route.params
      if (!poolId) return

      const shoulddiscard = confirm('Weet je zeker dat je deze pool wilt verwijderen?')
      if (!shoulddiscard) return

      const db = firebase.firestore()

      const b = db.batch()

      const pool = db.collection('pools').doc(poolId)
      const form = db.collection('pools').doc(poolId).collection('forms').doc('form')
      b.delete(form)
      b.delete(pool)
      await b.commit()

      commit('discard')

      router.push({ path: '/your-pools' })
    },

    async listUserPools ({ commit }) {
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
