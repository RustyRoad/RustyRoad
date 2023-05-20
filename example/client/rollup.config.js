import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import { terser } from 'rollup-plugin-terser';
import nodePolyfills from 'rollup-plugin-node-polyfills';
import external from "rollup-plugin-peer-deps-external";

export default {
  input: 'dist/server/entry-server.mjs',
  output: {
    file: 'dist/server/entry-server.mjs',
    format: 'es',
    sourcemap: true,
  },
  plugins: [
    external({
      includeDependencies: true
    }),
    nodePolyfills(),
    resolve({
      extensions: ['.mjs', '.js', '.jsx', '.json']
    }),

    commonjs(),
    terser(),
  ],
};
