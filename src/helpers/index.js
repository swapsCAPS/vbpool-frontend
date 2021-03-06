import allGames from '../assets/games.json'
import firebase from 'firebase/app'

export const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

export const fbAuthObservablePromiseWrapper = () => new Promise((resolve, reject) => {
  const unsub = firebase.auth().onAuthStateChanged(async (user, error) => {
    unsub()
    if (error) return reject(error)
    if (!user) return resolve()
    let token
    try {
      token = await firebase.auth().currentUser.getIdTokenResult()
    } catch (error) { return reject(error) }
    user.role = token.claims.role
    resolve(user)
  })
})

export const vbpStore = {
  save: (key, data) => {
    window.localStorage.setItem(key, JSON.stringify(data))
  },
  load: (key) => {
    let data
    const stored = window.localStorage.getItem(key)
    if (!stored) return console.log(`No data for ${key}`)
    try {
      data = JSON.parse(stored)
    } catch (error) {
      return console.error(`Could not parse ${key} from store`)
    }
    return data
  },
  delete: (key) => {
    window.localStorage.removeItem(key)
  },
}

export const getDefaultData = () => {
  return {
    page1: {
      meta: {
        poolName: '',
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
  }
}
