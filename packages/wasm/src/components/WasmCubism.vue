<template>
  <div class="wasm-cubism">
    hi
  </div>
</template>

<script lang="ts">
import init from 'cubism'
import { ref, defineComponent } from 'vue'

// init wasm
init('./cubism.wasm').then((wasm) => {
  wasm.run()
})

export default defineComponent({
  name: 'WasmCubism',
  setup: () => {

  },
})
</script>
