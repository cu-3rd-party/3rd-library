<script setup>
import ContentCard from './ContentCard.vue'

defineProps({
  items: { type: Array, default: () => [] },
  selectedId: { type: String, default: '' },
  loading: { type: Boolean, default: false },
})

const emit = defineEmits(['open'])
</script>

<template>
  <div class="content-grid">
    <div v-if="loading" class="panel">Loading library...</div>
    <div v-else-if="items.length === 0" class="panel">No uploads yet. Be the first to share.</div>
    <ContentCard
      v-for="item in items"
      :key="item.id"
      :item="item"
      :active="item.id === selectedId"
      @open="emit('open', $event)"
    />
  </div>
</template>
