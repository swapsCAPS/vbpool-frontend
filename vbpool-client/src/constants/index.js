export const actionCodeSettings = {
  url:             'https://vbpool.web.app',
  handleCodeInApp: true,
  iOS:             {
    bundleId: 'com.example.ios',
  },
  android: {
    packageName:    'com.example.android', // TODO gmail?
    installApp:     true,
    minimumVersion: '12',
  },
}

// eslint-disable-next-line no-useless-escape
export const emailRE = /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/

export const firebaseConfig = {
  apiKey:            process.env.VUE_APP_API_KEY,
  authDomain:        process.env.VUE_APP_AUTH_DOMAIN,
  databaseURL:       process.env.VUE_APP_DATABASE_URL,
  projectId:         process.env.VUE_APP_PROJECT_ID,
  storageBucket:     process.env.VUE_APP_STORAGE_BUCKET,
  messagingSenderId: process.env.VUE_APP_MESSAGING_SENDER_ID,
  appId:             process.env.VUE_APP_APP_ID,
  measurementId:     process.env.VUE_APP_MEASUREMENT_ID,
}

export const STORE_EMAIL_KEY = 'emailForSignIn'
