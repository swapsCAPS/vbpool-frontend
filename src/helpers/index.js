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
