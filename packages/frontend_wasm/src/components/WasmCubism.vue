<template>
  <div class="wasm-cubism">
  </div>
</template>

<script lang="ts">
import init, { run } from 'cubism'
import { ref, defineComponent } from 'vue'

export default defineComponent({
  name: 'WasmCubism',
  setup: () => {
    init()
    return { run }
  },
})
</script>
