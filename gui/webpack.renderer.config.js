/* eslint-disable @typescript-eslint/no-var-requires */
const rules = require('./webpack.rules');
const plugins = require('./webpack.plugins');
const path = require("path");

rules.push({
  test: /\.css$/,
  use: [{ loader: 'style-loader' }, { loader: 'css-loader' }],
});

rules.push({
  test: /\.png$/,
  use: 'file-loader'
});

module.exports = {
  module: {
    rules,
  },
  plugins: plugins,
  resolve: {
    extensions: ['.js', '.ts', '.jsx', '.tsx', '.css'],
    alias: {
      '@assets': path.resolve(__dirname, "src/assets"),
      '@common': path.resolve(__dirname, "src/common"),
      '@backend': path.resolve(__dirname, "src/backend"),
      '@frontend': path.resolve(__dirname, "src/frontend"),
    }
  },
};
