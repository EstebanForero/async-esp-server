import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

export default defineConfig({
  plugins: [solid()],
  build: {
    rollupOptions: {
      output: {
        entryFileNames: 'index.js',
        assetFileNames: 'index.[ext]'
      }
    },
    minify: 'terser',
    terserOptions: {
      compress: {
        defaults: true,
        ecma: 2015,
        module: true,
        passes: 10,
        keep_classnames: false,
        keep_fargs: false,
        keep_fnames: false,
        toplevel: true,
        unsafe: true,
        unsafe_arrows: true,
        drop_console: true
      },
      mangle: {
        module: true,
        toplevel: true
      },
      format: {
        comments: false,
        ecma: 2015,
        inline_script: true,
        semicolons: true,
        quote_style: 0,
        wrap_func_args: true
      }
    }
  }
})
