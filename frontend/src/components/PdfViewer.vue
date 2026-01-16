<script setup>
defineProps({
  title: { type: String, default: '' },
  blobUrl: { type: String, default: '' },
  loading: { type: Boolean, default: false },
})
</script>

<template>
  <section class="panel">
    <div class="section-title">Preview</div>
    <div class="viewer">
      <div v-if="loading" class="status-row">Loading PDF...</div>
      <div v-else-if="!blobUrl" class="status-row">Pick an item to preview it here.</div>
      <iframe v-else :title="title" :src="blobUrl"></iframe>
    </div>
  </section>
</template>
