import * as firebase from 'firebase/app'

const actionCodeSettings = {
  url:             `${process.env.VUE_APP_HOST_URL}/#/verify-email`,
  handleCodeInApp: true,
}

export const sendSignInLink = (email) => {
  return firebase.auth().sendSignInLinkToEmail(email, actionCodeSettings)
}

export const logOut = () => firebase.auth().signOut()
