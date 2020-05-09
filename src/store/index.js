import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    user: {
      isLoggedIn: false,
      email:      '',
    },
  },
  mutations: {
    setLoggedIn (state, value) {
      state.user.isLoggedIn = value
    },
    setEmail (state, value) {
      state.user.email = value
    },
  },
})

export default store
