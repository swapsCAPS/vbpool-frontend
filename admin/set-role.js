const admin = require('firebase-admin')

console.log('process.env.VUE_APP_DATABASE_URL', process.env.VUE_APP_DATABASE_URL)

const { 2: email, 3: role } = process.argv

const roles = [ 'admin', 'user' ]

if (!roles.includes(role)) return console.error(`Not a valid role '${role}'`)

admin.initializeApp({
  credential:  admin.credential.cert(require('../.firebase-admin-key.json')),
  databaseURL: process.env.VUE_APP_DATABASE_URL,
})

// Lookup the user associated with the specified uid.
admin.auth().getUserByEmail(email)
  .then(user => {
    if (!user.emailVerified) throw new Error('user not verified')

    console.log('user', user)
    return user
  }).then(user => {
    return admin.auth().setCustomUserClaims(user.uid, {
      role: 'admin',
    })
  }).catch(console.error)
