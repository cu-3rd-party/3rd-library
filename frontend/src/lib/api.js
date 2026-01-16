import { getOrCreateAnonymousId, getToken } from './storage'

const API_BASE = import.meta.env.VITE_API_BASE || 'http://localhost:8080'

async function request(path, options = {}) {
  const response = await fetch(`${API_BASE}${path}`, options)
  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Request failed with ${response.status}`)
  }
  return response
}

export async function postJson(path, payload, token = '') {
  const response = await request(path, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify(payload),
  })
  return response.json()
}

export async function getJson(path, token = '') {
  const response = await request(path, {
    headers: token ? { Authorization: `Bearer ${token}` } : {},
  })
  return response.json()
}

export async function getPdf(path, token = '') {
  const response = await request(path, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      'X-Anonymous-Id': getOrCreateAnonymousId(),
    },
  })
  return response.blob()
}

export async function uploadPdf(path, formData, token = '') {
  const response = await request(path, {
    method: 'POST',
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: formData,
  })
  return response.json()
}

export function apiBase() {
  return API_BASE
}

export function authToken() {
  return getToken()
}
