import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    target: 'es2015',
    outDir: 'dist/client', // Output directory for the client bundle
    ssr: 'src/entry-server.tsx', // Entry file for the server bundle
    minify: false,
    rollupOptions: {
      input: {
        server: 'src/entry-server.tsx', // Entry file for the server bundle (Rollup input)
      },
      external: ['react', 'react-dom'], // External dependencies for the server bundle
      output: {
        format: 'es', // Output format for the server bundle (Rollup output)
        exports: 'auto', // Automatically handle exports
      },
    },
  },
})
