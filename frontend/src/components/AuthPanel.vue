<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  user: { type: Object, default: null },
  busy: { type: Boolean, default: false },
  error: { type: String, default: '' },
})

const emit = defineEmits(['login', 'register', 'logout'])

const mode = ref('login')
const name = ref('')
const email = ref('')
const password = ref('')

watch(
  () => props.user,
  (value) => {
    if (value) {
      name.value = ''
      email.value = ''
      password.value = ''
    }
  }
)

function submit() {
  if (mode.value === 'register') {
    emit('register', { name: name.value, email: email.value, password: password.value })
    return
  }
  emit('login', { email: email.value, password: password.value })
}
</script>

<template>
  <section class="panel">
    <div class="section-head">
      <div>
        <div class="section-title">Account</div>
        <div class="status-row">{{ user ? `Signed in as ${user.name}` : 'Sign in to upload' }}</div>
      </div>
      <button v-if="user" class="btn" type="button" @click="emit('logout')">Log out</button>
    </div>

    <div v-if="!user" class="form">
      <div class="split-actions">
        <button class="btn" type="button" :disabled="mode === 'login'" @click="mode = 'login'">
          Login
        </button>
        <button class="btn" type="button" :disabled="mode === 'register'" @click="mode = 'register'">
          Register
        </button>
      </div>

      <div v-if="mode === 'register'" class="field">
        <label>Name</label>
        <input v-model="name" type="text" placeholder="Ada Lovelace">
      </div>
      <div class="field">
        <label>Email</label>
        <input v-model="email" type="email" placeholder="you@campus.edu">
      </div>
      <div class="field">
        <label>Password</label>
        <input v-model="password" type="password" placeholder="********">
      </div>
      <div class="split-actions">
        <button class="btn btn-accent" type="button" :disabled="busy" @click="submit">
          {{ mode === 'register' ? 'Create account' : 'Login' }}
        </button>
      </div>
      <div v-if="error" class="status-row">{{ error }}</div>
    </div>
  </section>
</template>
