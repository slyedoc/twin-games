import path from 'path'
import fs from 'fs'
import { defineConfig } from 'vite'
import Vue from '@vitejs/plugin-vue'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import Components from 'unplugin-vue-components/vite'
import AutoImport from 'unplugin-auto-import/vite'
import { VitePWA } from 'vite-plugin-pwa'
import Inspect from 'vite-plugin-inspect'
import ViteRsw from 'vite-plugin-rsw'

export default defineConfig({
    resolve: {
        alias: {
            '~/': `${path.resolve(__dirname, 'src')}/`,
        },
    },
    plugins: [
        // https://github.com/lencx/vite-plugin-rsw#plugin-options
        // https://rustwasm.github.io/docs/wasm-pack/commands/build.html
        ViteRsw({
            root: 'games',
            profile: 'dev', // 'dev' | 'release' | 'profiling'
            target: 'web', // 'bundler' | 'web' | 'nodejs' | 'no-modules'
            crates: getDirectories('games').map((dir) => {
                return {
                    name: `${dir}`,
                    outDir: `${path.resolve(__dirname, 'pkg')}/${dir}`,
                    unwatch: [`${path.resolve(__dirname, 'pkg')}`]
                }
            }),
        }),

        // https://github.com/vitejs/vite/tree/main/packages/plugin-vue#readme
        Vue(),

        // https://github.com/hannoeru/vite-plugin-pages
        Pages(),

        // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
        Layouts(),

        // https://github.com/antfu/unplugin-auto-import
        AutoImport({
            imports: [
                'vue',
                'vue-router',
                '@vueuse/head',
                '@vueuse/core',
            ],
            dts: 'src/auto-imports.d.ts',
        }),

        // https://github.com/antfu/unplugin-vue-components
        Components({
            // allow auto load markdown components under `./src/components/`
            extensions: ['vue'],

            // allow auto import and register components used in markdown
            include: [/\.vue$/, /\.vue\?vue/],

            // custom resolvers
            resolvers: [
                // auto import icons
                // https://github.com/antfu/unplugin-icons
                IconsResolver({
                    componentPrefix: '',
                    // enabledCollections: ['carbon']
                }),
                // VuetifyResolver(),
            ],

            dts: 'src/components.d.ts',
        }),

        // https://github.com/antfu/unplugin-icons
        Icons({
            autoInstall: true,
        }),

        // https://github.com/antfu/vite-plugin-pwa
        VitePWA({
            registerType: 'autoUpdate',
            includeAssets: ['favicon.svg', 'robots.txt', 'safari-pinned-tab.svg'],
            manifest: {
                name: 'Vitesse',
                short_name: 'Vitesse',
                theme_color: '#ffffff',
                icons: [
                    {
                        src: '/pwa-192x192.png',
                        sizes: '192x192',
                        type: 'image/png',
                    },
                    {
                        src: '/pwa-512x512.png',
                        sizes: '512x512',
                        type: 'image/png',
                    },
                    {
                        src: '/pwa-512x512.png',
                        sizes: '512x512',
                        type: 'image/png',
                        purpose: 'any maskable',
                    },
                ],
            },
        }),

        // https://github.com/antfu/vite-plugin-inspect
        Inspect({
            // change this to enable inspect for debugging
            enabled: false,
        }),
    ],
    server: {
        fs: {
            strict: true,
        },
    },

    // https://github.com/antfu/vite-ssg
    ssgOptions: {
        script: 'async',
        formatting: 'minify',
    },

    optimizeDeps: {
        include: [
            'vue',
            'vue-router',
            '@vueuse/core',
            '@vueuse/head',
        ],
        exclude: [
            'vue-demi',
            'cubism',
            'disco',
        ],
    },
})

function getDirectories(path: string) {
    return fs.readdirSync(path).filter((file) => {
        if (!file.startsWith('.'))
            return fs.statSync(`${path}/${file}`).isDirectory()
        return false
    })
}

