import { getAuth } from 'firebase/auth'

const actionCodeSettings = {
  url:             `${process.env.VUE_APP_HOST_URL}/#/verify-email`,
  handleCodeInApp: true,
}

export const sendSignInLink = (email) => {
  return getAuth().sendSignInLinkToEmail(email, actionCodeSettings)
}

export const logOut = () => getAuth().signOut()
