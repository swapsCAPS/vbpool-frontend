import * as firebase from 'firebase/app'
import VueRouter from 'vue-router'

import SignIn from '../components/SignIn.vue'
import VerifyEmail from '../components/VerifyEmail.vue'
import YourPools from '../components/YourPools.vue'
import Admin from '../components/Admin.vue'
import PoolForm from '../components/PoolForm.vue'
import store from '../store'

const routes = [
  { path: '/', name: 'root', redirect: '/form' },

  {
    path:        '/signin',
    name:        'signin',
    component:   SignIn,
    beforeEnter: (to, from, next) => {
      if (store.state.user.isLoggedIn) return next({ name: 'form' })
      next()
    },
  },

  { path: '/verify-email', name: 'verify-email', component: VerifyEmail },

  {
    path:        '/form',
    name:        'form',
    component:   PoolForm,
    beforeEnter: async (to, from, next) => {
      store.commit('discard')
      next()
    },
  },

  {
    path:        '/form/:poolId',
    name:        'edit-form',
    component:   PoolForm,
    beforeEnter: async (to, from, next) => {
      try {
        await store.dispatch('fetchAndSetPool', to.params.poolId)
      } catch (e) {
        console.error(e)
        return next({ path: '/form' })
      }
      next()
    },
  },

  { path: '/your-pools', name: 'your-pools', component: YourPools },
  { path: '/admin', name: 'admin', component: Admin },
]

const router = new VueRouter({ routes })

router.beforeEach((to, from, next) => {
  if (to.name === 'signin' || to.name === 'verify-email') return next()
  if (!firebase.auth().currentUser) return next({ name: 'signin' })
  next()
})

export default router
