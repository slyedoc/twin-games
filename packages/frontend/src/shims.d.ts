/* eslint-disable import/no-duplicates */

declare interface Window {
  // extend the window
}

// Fix for Cannot find module './App.vue'
// https://github.com/vuejs/vue-next-webpack-preview/issues/5
declare module '*.vue' {
  import { ComponentOptions } from 'vue'
  const component: ComponentOptions
  export default component
}
