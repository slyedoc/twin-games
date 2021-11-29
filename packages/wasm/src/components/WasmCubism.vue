<template>
  <div class="wasm-cubism">
  </div>
</template>

<script lang="ts">
import init from 'cubism'
import { ref, defineComponent } from 'vue'

// WILL BLOW UP
// init('cubism/cubism_bg.wasm').then((wasm) => {
//   wasm.run()
// })

export default defineComponent({
  name: 'WasmCubism',
  setup: () => {

  },

})
</script>
