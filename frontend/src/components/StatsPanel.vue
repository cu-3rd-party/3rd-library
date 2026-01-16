<script setup>
defineProps({
  stats: { type: Object, default: null },
  loading: { type: Boolean, default: false },
})
</script>

<template>
  <section class="panel">
    <div class="section-title">Engagement</div>
    <div v-if="loading" class="status-row">Loading stats...</div>
    <div v-else-if="!stats" class="status-row">Select a PDF to see interactions.</div>
    <div v-else class="stats-grid">
      <div class="stat-card">
        <div class="stat-value">{{ stats.views }}</div>
        <div class="status-row">Views</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ stats.downloads }}</div>
        <div class="status-row">Downloads</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ stats.unique_users }}</div>
        <div class="status-row">Unique readers</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">
          {{ stats.last_interaction_at ? new Date(stats.last_interaction_at * 1000).toLocaleDateString() : '-' }}
        </div>
        <div class="status-row">Last interaction</div>
      </div>
    </div>
  </section>
</template>
