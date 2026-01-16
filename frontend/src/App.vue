<script setup>
import { computed, onMounted, reactive, ref } from 'vue'

import AuthPanel from './components/AuthPanel.vue'
import ContentList from './components/ContentList.vue'
import PdfViewer from './components/PdfViewer.vue'
import StatsPanel from './components/StatsPanel.vue'
import UploadPanel from './components/UploadPanel.vue'
import { apiBase, getJson, getPdf, postJson, uploadPdf } from './lib/api'
import { getOrCreateAnonymousId, getToken, getUser, setToken, setUser } from './lib/storage'

const auth = reactive({
  user: getUser(),
  token: getToken(),
  busy: false,
  error: '',
})

const library = reactive({
  items: [],
  loading: false,
  pageToken: '',
  nextToken: '',
  history: [],
})

const selection = reactive({
  item: null,
  blobUrl: '',
  loading: false,
  stats: null,
  statsLoading: false,
})

const uploadState = reactive({
  busy: false,
  error: '',
  message: '',
})

const apiStatus = ref('')

const selectedId = computed(() => (selection.item ? selection.item.id : ''))

onMounted(() => {
  getOrCreateAnonymousId()
  loadContents()
})

async function loadContents(reset = false) {
  if (library.loading) return
  library.loading = true
  apiStatus.value = ''
  try {
    if (reset) {
      library.pageToken = ''
      library.history = []
    }
    const query = new URLSearchParams()
    query.set('page_size', '9')
    if (library.pageToken) {
      query.set('page_token', library.pageToken)
    }
    const data = await getJson(`/contents?${query.toString()}`)
    library.items = data.items || []
    library.nextToken = data.next_page_token || ''
  } catch (error) {
    apiStatus.value = error.message
  } finally {
    library.loading = false
  }
}

async function nextPage() {
  if (!library.nextToken) return
  library.history.push(library.pageToken)
  library.pageToken = library.nextToken
  await loadContents()
}

async function prevPage() {
  if (library.history.length === 0) return
  library.pageToken = library.history.pop() || ''
  await loadContents()
}

async function handleRegister(payload) {
  auth.busy = true
  auth.error = ''
  try {
    const data = await postJson('/auth/register', payload)
    auth.user = data.user
    auth.token = data.token
    setUser(data.user)
    setToken(data.token)
  } catch (error) {
    auth.error = error.message
  } finally {
    auth.busy = false
  }
}

async function handleLogin(payload) {
  auth.busy = true
  auth.error = ''
  try {
    const data = await postJson('/auth/login', payload)
    auth.user = data.user
    auth.token = data.token
    setUser(data.user)
    setToken(data.token)
  } catch (error) {
    auth.error = error.message
  } finally {
    auth.busy = false
  }
}

function handleLogout() {
  auth.user = null
  auth.token = ''
  setUser(null)
  setToken('')
}

async function viewItem(item) {
  selection.loading = true
  selection.item = item
  selection.stats = null
  selection.statsLoading = true
  apiStatus.value = ''
  if (selection.blobUrl) {
    URL.revokeObjectURL(selection.blobUrl)
    selection.blobUrl = ''
  }
  try {
    const blob = await getPdf(`/contents/${item.id}`, auth.token)
    selection.blobUrl = URL.createObjectURL(blob)
    await loadStats(item.id)
  } catch (error) {
    apiStatus.value = error.message
  } finally {
    selection.loading = false
  }
}

async function downloadItem(item) {
  const url = `${apiBase()}/contents/${item.id}/download`
  window.open(url, '_blank')
  await loadStats(item.id)
}

async function loadStats(contentId) {
  selection.statsLoading = true
  try {
    const data = await getJson(`/contents/${contentId}/stats`)
    selection.stats = data
  } catch (error) {
    apiStatus.value = error.message
  } finally {
    selection.statsLoading = false
  }
}

async function handleUpload(payload) {
  uploadState.busy = true
  uploadState.error = ''
  uploadState.message = ''
  try {
    const formData = new FormData()
    formData.append('title', payload.title || '')
    formData.append('description', payload.description || '')
    formData.append('file', payload.file)
    const data = await uploadPdf('/contents', formData, auth.token)
    uploadState.message = `Uploaded ${data.title}`
    await loadContents(true)
  } catch (error) {
    uploadState.error = error.message
  } finally {
    uploadState.busy = false
  }
}
</script>

<template>
  <div class="page">
    <header class="topbar">
      <div class="brand">
        <div class="brand-badge">3L</div>
        Third Library
      </div>
      <div class="nav-meta">
        <span>API {{ apiBase() }}</span>
        <span>PDF exchange for academic discoveries</span>
      </div>
    </header>

    <section class="hero">
      <div class="hero-copy">
        <div class="hero-title">Share work that moves the classroom forward.</div>
        <div class="hero-subtitle">
          Upload papers, lab notes, and decks. Scan the library in minutes, save what matters, and
          track what your cohort is actually reading.
        </div>
        <div class="pill-row">
          <span class="pill">JWT secured uploads</span>
          <span class="pill">Paginated library</span>
          <span class="pill">Live interaction stats</span>
          <span class="pill">PDF streaming</span>
        </div>
      </div>
      <AuthPanel
        :user="auth.user"
        :busy="auth.busy"
        :error="auth.error"
        @login="handleLogin"
        @register="handleRegister"
        @logout="handleLogout"
      />
    </section>

    <section class="section">
      <div class="section-head">
        <div>
          <div class="section-title">Library</div>
          <div class="status-row">Browse, preview, and download academic PDFs.</div>
        </div>
        <div class="pagination">
          <button class="btn" type="button" :disabled="library.history.length === 0" @click="prevPage">
            Previous
          </button>
          <button class="btn" type="button" :disabled="!library.nextToken" @click="nextPage">
            Next
          </button>
        </div>
      </div>
      <ContentList
        :items="library.items"
        :selected-id="selectedId"
        :loading="library.loading"
        @view="viewItem"
        @download="downloadItem"
      />
    </section>

    <section class="grid-two">
      <UploadPanel
        :user="auth.user"
        :busy="uploadState.busy"
        :error="uploadState.error"
        :message="uploadState.message"
        @upload="handleUpload"
      />
      <PdfViewer
        :title="selection.item?.title || ''"
        :blob-url="selection.blobUrl"
        :loading="selection.loading"
      />
    </section>

    <section class="grid-two">
      <StatsPanel :stats="selection.stats" :loading="selection.statsLoading" />
      <div class="panel">
        <div class="section-title">Live notes</div>
        <div class="status-row">
          Start by selecting a PDF, then share the download link with your cohort. The stats panel
          updates as they read.
        </div>
        <div v-if="apiStatus" class="status-row">Status: {{ apiStatus }}</div>
      </div>
    </section>

    <footer class="footer">
      Built for the Third Library microservices stack. Uploads require login. Viewing is open to all.
    </footer>
  </div>
</template>
