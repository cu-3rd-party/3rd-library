<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  user: { type: Object, default: null },
  busy: { type: Boolean, default: false },
  error: { type: String, default: '' },
  message: { type: String, default: '' },
})

const emit = defineEmits(['upload'])

const title = ref('')
const description = ref('')
const file = ref(null)

watch(
  () => props.user,
  (value) => {
    if (!value) {
      title.value = ''
      description.value = ''
      file.value = null
    }
  }
)

function onFileChange(event) {
  const selected = event.target.files[0]
  file.value = selected || null
}

function submit() {
  if (!file.value) {
    return
  }
  emit('upload', {
    title: title.value,
    description: description.value,
    file: file.value,
  })
}
</script>

<template>
  <section class="panel">
    <div class="section-head">
      <div>
        <div class="section-title">Upload a discovery</div>
        <div class="status-row">PDFs only. Share notes, research, and reference decks.</div>
      </div>
    </div>

    <div v-if="!user" class="status-row">Login to upload your work.</div>
    <div v-else class="form">
      <div class="field">
        <label>Title</label>
        <input v-model="title" type="text" placeholder="Quantum Error Correction Summary">
      </div>
      <div class="field">
        <label>Description</label>
        <textarea v-model="description" rows="3" placeholder="Key findings, context, or dataset notes."></textarea>
      </div>
      <div class="field">
        <label>PDF file</label>
        <input type="file" accept="application/pdf" @change="onFileChange">
      </div>
      <div class="split-actions">
        <button class="btn btn-accent" type="button" :disabled="busy || !file" @click="submit">
          Upload PDF
        </button>
      </div>
      <div v-if="message" class="status-row">{{ message }}</div>
      <div v-if="error" class="status-row">{{ error }}</div>
    </div>
  </section>
</template>
