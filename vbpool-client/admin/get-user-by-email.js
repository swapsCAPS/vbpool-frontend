const admin = require('firebase-admin')

const { 2: email } = process.argv

admin.initializeApp({
  credential:  admin.credential.cert(require('../.firebase-admin-key.json')),
  databaseURL: process.env.VUE_APP_DATABASE_URL,
})

// Lookup the user associated with the specified uid.
admin.auth().getUserByEmail(email)
  .then(user => {
    console.log('user', user)
    return user
  })
