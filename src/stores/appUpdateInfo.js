import { ref } from 'vue'
import { defineStore } from 'pinia'

export const appUpdateInfoStore = defineStore('updateInfo', () => {
  const updateAvailable = ref(false);
  const manifest = ref("");

  return { updateAvailable, manifest };
})
