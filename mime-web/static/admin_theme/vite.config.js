import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite'


export default defineConfig({
    plugins: [
        tailwindcss(),
    ],
    build: {
        manifest: true,
        outDir: 'dist',
        emptyOutDir: true,
        rollupOptions: {
            input: './src/index.js',
            output: {
                entryFileNames: `[name].js`,
                chunkFileNames: `[name].js`,
                assetFileNames: `[name].[ext]`
            }
        }
    }
});
