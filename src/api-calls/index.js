import * as firebase from 'firebase/app'

const actionCodeSettings = {
  url:             process.env.VBPOOL_APP_URL,
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

export const sendSignInLink = (email) => {
  firebase.auth().sendSignInLink(email, actionCodeSettings)
}
