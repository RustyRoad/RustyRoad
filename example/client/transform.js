const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['dist/server/entry-server.js'], // Update this path to the location of your server entry file
  outfile: 'path/to/server/entry-server.cjs', // Update this path to where you want the transformed file to be saved
  format: 'cjs', // Output format as CommonJS
  platform: 'node', // Target platform is Node.js
  bundle: true, // Bundle all dependencies into a single file
}).catch(() => process.exit(1));
