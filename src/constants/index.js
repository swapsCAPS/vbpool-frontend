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
