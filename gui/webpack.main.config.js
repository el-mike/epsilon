const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = {
  /**
   * This is the main entry point for your application, it's the first file
   * that runs in the main process.
   */
  entry: './src/main.ts',
  // Put your normal webpack config below here
  module: {
    rules: require('./webpack.rules'),
  },
  resolve: {
    extensions: ['.js', '.ts', '.jsx', '.tsx', '.css', '.json'],
    alias: {
      '@assets': path.resolve(__dirname, "src/assets"),
      '@common': path.resolve(__dirname, "src/common"),
      '@backend': path.resolve(__dirname, "src/backend"),
      '@frontend': path.resolve(__dirname, "src/frontend"),
    }
  },
  externals: [nodeExternals()]
};
