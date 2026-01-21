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
const currentScreen = ref('library')
const anonId = ref('')
const commentDraft = ref('')

const engagement = reactive({
  likes: {},
  liked: {},
  comments: {},
})

const likeCount = computed(() =>
  selection.item ? engagement.likes[selection.item.id] || 0 : 0
)
const liked = computed(() =>
  selection.item ? Boolean(engagement.liked[selection.item.id]) : false
)
const commentList = computed(() =>
  selection.item ? engagement.comments[selection.item.id] || [] : []
)
const currentAuthor = computed(() => {
  if (auth.user?.name) return auth.user.name
  if (anonId.value) return `Guest ${anonId.value.slice(0, 6)}`
  return 'Guest'
})

onMounted(() => {
  anonId.value = getOrCreateAnonymousId()
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
  currentScreen.value = 'detail'
  commentDraft.value = ''
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

function backToLibrary() {
  currentScreen.value = 'library'
  selection.item = null
  selection.stats = null
  selection.loading = false
  selection.statsLoading = false
  commentDraft.value = ''
  if (selection.blobUrl) {
    URL.revokeObjectURL(selection.blobUrl)
    selection.blobUrl = ''
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

function toggleLike() {
  if (!selection.item) return
  const contentId = selection.item.id
  const wasLiked = Boolean(engagement.liked[contentId])
  engagement.liked[contentId] = !wasLiked
  const nextCount = (engagement.likes[contentId] || 0) + (wasLiked ? -1 : 1)
  engagement.likes[contentId] = Math.max(0, nextCount)
}

function addComment() {
  if (!selection.item) return
  const body = commentDraft.value.trim()
  if (!body) return
  const contentId = selection.item.id
  if (!engagement.comments[contentId]) {
    engagement.comments[contentId] = []
  }
  const commentId =
    typeof crypto !== 'undefined' && crypto.randomUUID
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random()}`
  engagement.comments[contentId].unshift({
    id: commentId,
    author: currentAuthor.value,
    body,
    createdAt: Date.now(),
  })
  commentDraft.value = ''
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
      <div class="topbar-actions">
        <div class="nav-meta">
          <span>API {{ apiBase() }}</span>
          <span>{{ auth.user ? `Signed in as ${auth.user.name}` : 'Guest access' }}</span>
        </div>
        <button v-if="currentScreen === 'detail'" class="btn btn-ghost" type="button" @click="backToLibrary">
          Back to library
        </button>
      </div>
    </header>

    <section v-if="currentScreen === 'library'" class="library-screen">
      <div class="section-head">
        <div>
          <div class="section-title">Publication tiles</div>
          <div class="status-row">Click any publication to open the full preview.</div>
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
      <div class="library-layout">
        <div class="library-main">
          <ContentList
            :items="library.items"
            :selected-id="selectedId"
            :loading="library.loading"
            @open="viewItem"
          />
          <div v-if="apiStatus" class="status-row">Status: {{ apiStatus }}</div>
        </div>
        <div class="library-aside">
          <UploadPanel
            :user="auth.user"
            :busy="uploadState.busy"
            :error="uploadState.error"
            :message="uploadState.message"
            @upload="handleUpload"
          />
          <AuthPanel
            :user="auth.user"
            :busy="auth.busy"
            :error="auth.error"
            @login="handleLogin"
            @register="handleRegister"
            @logout="handleLogout"
          />
        </div>
      </div>
    </section>

    <section v-else class="detail-screen">
      <div class="detail-header">
        <div>
          <div class="detail-title">{{ selection.item?.title || 'Publication' }}</div>
          <div class="detail-subtitle">{{ selection.item?.description || 'No description provided.' }}</div>
          <div class="detail-meta">
            <span>Owner {{ selection.item?.owner_id?.slice(0, 8) || '-' }}</span>
            <span>{{ selection.item ? (selection.item.size_bytes / 1024 / 1024).toFixed(2) : '0.00' }} MB</span>
            <span>{{ selection.item ? new Date(selection.item.created_at * 1000).toLocaleDateString() : '-' }}</span>
          </div>
        </div>
        <div class="detail-actions">
          <button class="btn" type="button" :disabled="!selection.item" @click="downloadItem(selection.item)">
            Download PDF
          </button>
        </div>
      </div>
      <div class="detail-layout">
        <PdfViewer
          :title="selection.item?.title || ''"
          :blob-url="selection.blobUrl"
          :loading="selection.loading"
        />
        <div class="detail-side">
          <section class="panel">
            <div class="section-title">Engagement</div>
            <div class="like-row">
              <button class="btn btn-like" type="button" @click="toggleLike">
                {{ liked ? 'Liked' : 'Like' }}
              </button>
              <span class="status-row">{{ likeCount }} likes</span>
            </div>
          </section>
          <StatsPanel :stats="selection.stats" :loading="selection.statsLoading" />
          <section class="panel comments-panel">
            <div class="section-title">Comments</div>
            <div v-if="commentList.length === 0" class="status-row">No comments yet. Start the discussion.</div>
            <div v-else class="comment-list">
              <div v-for="comment in commentList" :key="comment.id" class="comment-card">
                <div class="comment-meta">
                  <span class="comment-author">{{ comment.author }}</span>
                  <span class="comment-date">{{ new Date(comment.createdAt).toLocaleString() }}</span>
                </div>
                <div class="comment-body">{{ comment.body }}</div>
              </div>
            </div>
            <div class="comment-form">
              <textarea
                v-model="commentDraft"
                rows="3"
                placeholder="Leave a note about this publication."
              ></textarea>
              <button class="btn btn-accent" type="button" :disabled="!commentDraft.trim()" @click="addComment">
                Post comment
              </button>
            </div>
          </section>
        </div>
      </div>
    </section>

    <footer class="footer">
      Built for the Third Library microservices stack. Uploads require login. Viewing is open to all.
    </footer>
  </div>
</template>
