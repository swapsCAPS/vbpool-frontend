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
  apiKey:            process.env.VBPOOL_API_KEY,
  authDomain:        process.env.VBPOOL_AUTH_DOMAIN,
  databaseURL:       process.env.VBPOOL_DATABASE_URL,
  projectId:         process.env.VBPOOL_PROJECT_ID,
  storageBucket:     process.env.VBPOOL_STORAGE_BUCKET,
  messagingSenderId: process.env.VBPOOL_MESSAGING_SENDER_ID,
  appId:             process.env.VBPOOL_APP_ID,
  measurementId:     process.env.VBPOOL_MEASUREMENT_ID,
}
