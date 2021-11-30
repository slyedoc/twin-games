<template>
  <div class="wasm">
    <div v-if="!gpu">
      WebGPU not supported! Please visit <a href="//webgpu.io">webgpu.io</a> to see the current implementation status.
    </div>
    {{ gpu }}
  </div>
</template>

<script lang="ts">

import init, { run } from 'cornell_box'
import { ref, defineComponent } from 'vue'

export default defineComponent({
  name: 'WasmCornellBox',
  props: {
    gpu: Boolean
  },
  setup: async (props) => {
    let gpu = (navigator as any).gpu;
    if (gpu) {
      console.log('GPU is supported!')
      let wasm = await init();
      console.log('Init done')
      wasm.run();
    }
    return {
      gpu
    }
  },
})
</script>
