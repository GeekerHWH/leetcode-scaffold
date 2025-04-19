const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');

/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config, { isServer }) => {
    // 仅在客户端构建时添加 Monaco 插件
    if (!isServer) {
      config.plugins.push(
        new MonacoWebpackPlugin({
          languages: ['javascript', 'typescript', 'html', 'css'],
          filename: 'static/[name].worker.js',
        })
      );
    }
    return config;
  },
}

module.exports = nextConfig;