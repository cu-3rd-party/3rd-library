const TOKEN_KEY = 'third-library-token'
const USER_KEY = 'third-library-user'
const ANON_KEY = 'third-library-anon'

export function getToken() {
  return localStorage.getItem(TOKEN_KEY) || ''
}

export function setToken(token) {
  if (token) {
    localStorage.setItem(TOKEN_KEY, token)
  } else {
    localStorage.removeItem(TOKEN_KEY)
  }
}

export function getUser() {
  const raw = localStorage.getItem(USER_KEY)
  return raw ? JSON.parse(raw) : null
}

export function setUser(user) {
  if (user) {
    localStorage.setItem(USER_KEY, JSON.stringify(user))
  } else {
    localStorage.removeItem(USER_KEY)
  }
}

export function getOrCreateAnonymousId() {
  let anon = localStorage.getItem(ANON_KEY)
  if (!anon) {
    anon = typeof crypto !== 'undefined' && crypto.randomUUID ? crypto.randomUUID() : `${Date.now()}-${Math.random()}`
    localStorage.setItem(ANON_KEY, anon)
  }
  return anon
}
