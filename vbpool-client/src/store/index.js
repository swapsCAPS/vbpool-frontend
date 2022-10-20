import _ from 'lodash'
import { createStore } from 'vuex'

import { getField, updateField } from 'vuex-map-fields'

import { getDefaultData } from '../helpers'

import { getFirestore, collection, addDoc, doc, setDoc, query, where, getDocs } from 'firebase/firestore'
import { getAuth } from 'firebase/auth'

import { useRouter } from 'vue-router'

const { page1, page2 } = getDefaultData()

const store = createStore({
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
    async createPool ({ state, commit }) {
      if (!state.form.page1.meta.poolName) return

      const db = getFirestore()

      commit('setSaving', true)
      const list = await getDocs(
        query(
          collection(db, 'pools'),
          where('userId', '==', getAuth().currentUser.uid),
        ),
      )
      if (list.docs.length >= 10) return alert('Je kunt niet meer dan 10 pools aanmaken')

      const userId = getAuth().currentUser.uid

      const pool = await addDoc(collection(db, 'pools'), {
        userId,
        meta: {
          poolName: state.form.page1.meta.poolName,
        },
        flags: {
          isPaid:     false,
          isComplete: false,
        },
      })

      const formRef = doc(db, [ 'pools', pool.id, 'forms', 'form' ])
      await setDoc(formRef, state.form)

      commit('setSaving', false)

      useRouter().push({ name: 'edit-form', params: { poolId: pool.id } })
    },

    async readPool ({ commit }, id) {
      if (!id) throw new Error('readPool called w/o an id')

      const db = getFirestore()

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
      console.log('router', useRouter)
      console.log('router', useRouter())
      console.log('router.currentRoute', useRouter().currentRoute)
      const { poolId } = useRouter().currentRoute.params
      if (!poolId) return

      if (state.isSaving) return // Maybe we should cancel the current and invoke the next save...

      commit('setSaving', true)

      const db = getFirestore()

      // TODO validation
      await db.collection('pools').doc(poolId).collection('forms').doc('form').set(state.form)

      setTimeout(() => {
        commit('setSaving', false)
      }, 1500)
    },

    async deletePool ({ commit }) {
      const { poolId } = useRouter().currentRoute.params
      if (!poolId) return

      const shoulddiscard = confirm('Weet je zeker dat je deze pool wilt verwijderen?')
      if (!shoulddiscard) return

      const db = getFirestore()

      const b = db.batch()

      const pool = db.collection('pools').doc(poolId)
      const form = db.collection('pools').doc(poolId).collection('forms').doc('form')
      b.delete(form)
      b.delete(pool)
      await b.commit()

      commit('discard')

      useRouter().push({ path: '/your-pools' })
    },

    async listUserPools ({ commit }) {
      const db = getFirestore()

      const response = await db
        .collection('pools')
        .where('userId', '==', getAuth().currentUser.uid)
        .get()

      commit('setUserPools', response.docs.map(doc => ({ id: doc.id, ...doc.data() })))
    },
  },
})

export default store
