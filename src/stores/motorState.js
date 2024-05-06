import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useMotorStore = defineStore('motor', () => {
  const currRps = ref(0.0);

  return { currRps };
})

// export const useMotorStore = defineStore('motor', {
//   state: () => ({ currRps: 0.0 }),
//   getters: {},
//   actions: {},
// })
