import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import ViteRsw from 'vite-plugin-rsw';

// https://vitejs.dev/config/
// https://github.com/lencx/vite-plugin-rsw#plugin-options
// https://rustwasm.github.io/docs/wasm-pack/commands/build.html
export default defineConfig({
    root: 'website',
    mode: 'development', //production
    publicDir: 'website/public',
    plugins: [
        vue(),
        ViteRsw({
            root: 'games',
            crates: [
                {
                    name: 'cubism',
                    outDir: '../../pkg/cubism'
                },
                
            ]
        }),
    ]
})
