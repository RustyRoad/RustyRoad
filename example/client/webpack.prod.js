const path = require('path');

module.exports = {
  mode: 'production',
  target: 'browserslist:defaults',
  entry: './dist/server/entry-server.js',
  output: {
    path: path.resolve(__dirname, 'dist/server'),
    filename: 'entry-server.mjs',
    library: 'MyLibrary', // Name of the library
    libraryTarget: 'module', // Output as ES module
    chunkFormat: 'esm', // Use ESM chunk format
    module: true, // Enable output.module
    environment: {
      module: true, // Enable output.environment.module
    },
    globalObject: 'this', // Ensure the library works in different environments
  },
  experiments: {
    outputModule: true, // Enable experiments.outputModule
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [
              ['@babel/preset-env', { modules: false }], // Output as ES modules
              '@babel/preset-react',
            ],
          },
        },
      },
    ],
  },
  resolve: {
    extensions: ['.js'],
    alias: {
      'react': path.resolve(__dirname, 'node_modules/react/index.js'),
      'react-dom': path.resolve(__dirname, 'node_modules/react-dom/index.js'),
    },
    fallback: {
      util: require.resolve("util/")
    },
  },
  target: 'node',
  externalsPresets: { node: true }, // Exclude Node.js built-in modules
};
