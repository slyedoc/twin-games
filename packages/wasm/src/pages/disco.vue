<script lang="ts">
import { defineComponent } from 'vue'
import init from 'disco'

export default defineComponent({
  name: 'HelloWasm',
  setup: () => {
    // init wasm
    // init('disco/disco_bg.wasm').then((wasm) => {
    //   wasm.run()
    // })
  },
})
</script>

<template>
  <v-card class="elevation-11">
    <v-card-title primary-title>
      Home
    </v-card-title>
    <v-card-text>
      Welcome to the start of Twin Games!

      <br />

      <tic-tac-toe-game></tic-tac-toe-game>
    </v-card-text>
  </v-card>
</template>

<route lang="yaml">
meta:
  layout: default
</route>
