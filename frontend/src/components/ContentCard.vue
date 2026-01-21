<script setup>
defineProps({
  item: { type: Object, required: true },
  active: { type: Boolean, default: false },
})

const emit = defineEmits(['open'])
</script>

<template>
  <article
    class="content-card"
    :style="active ? 'border-color: rgba(227, 97, 47, 0.6)' : ''"
    role="button"
    tabindex="0"
    @click="emit('open', item)"
    @keydown.enter="emit('open', item)"
  >
    <div class="content-title">{{ item.title }}</div>
    <div class="content-description">{{ item.description || 'No description' }}</div>
    <div class="content-meta">
      <span>Owner {{ item.owner_id.slice(0, 8) }}</span>
      <span>{{ (item.size_bytes / 1024 / 1024).toFixed(2) }} MB</span>
      <span>{{ new Date(item.created_at * 1000).toLocaleDateString() }}</span>
    </div>
    <div class="card-actions">
      <span class="card-cta">Open publication</span>
    </div>
  </article>
</template>
