import { getAuth, sendSignInLinkToEmail } from 'firebase/auth'

const actionCodeSettings = {
  url:             `${process.env.VUE_APP_HOST_URL}/verify-email`,
  handleCodeInApp: true,
}

export const sendSignInLink = (email) => {
  console.log('actionCodeSettings', actionCodeSettings)
  console.log('getAuth()', getAuth())
  return sendSignInLinkToEmail(getAuth(), email, actionCodeSettings)
}

export const logOut = () => getAuth().signOut()
