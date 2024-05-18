import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useMotorStore = defineStore('motor', () => {
  const currRps = ref(0.0);
  const isTesting = ref(false);
  const isConnected = ref(false);

  return { currRps, isTesting, isConnected };
})

// export const useMotorStore = defineStore('motor', {
//   state: () => ({ currRps: 0.0 }),
//   getters: {
//     get: (state) => state.currRps,
//   },
//   actions: {
//     update(rps) {
//       this.$state.currRps = rps;
//     }
//   },
// })
