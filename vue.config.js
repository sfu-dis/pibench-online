const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin')

module.exports = {
    configureWebpack: (conf) => {
        conf.entry['app'].push(require.resolve(`webpack-dev-server/client`) + '?http://home.haoxp.xyz:7001')
    },
    devServer: {
        port: 7001,
        disableHostCheck: true
    },
    chainWebpack: config => {
        config.plugin('monaco-editor').use(MonacoWebpackPlugin, [
            {
                // Languages are loaded on demand at runtime
                languages: ['json', 'javascript', 'html', 'xml']
            }
        ])
    }
}