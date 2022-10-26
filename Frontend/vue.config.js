const { defineConfig } = require('@vue/cli-service')
const NodePolyfillPlugin = require('node-polyfill-webpack-plugin')

module.exports = defineConfig({

  transpileDependencies: true,
  configureWebpack: {
    plugins: [new NodePolyfillPlugin()],

  },
  devServer:{
    proxy: {
      '/service/rest': {
        target: 'https://cloudflare-ipfs.com/ipfs/',
        ws: true,
        changeOrigin: true,
        pathRewrite: {

        }
      }
    }

  }


})
