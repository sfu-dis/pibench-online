module.exports = {
    configureWebpack: (conf) => {
        conf.entry['app'].push(require.resolve(`webpack-dev-server/client`) + '?http://home.haoxp.xyz:7001')
    },
    devServer: {
        port: 7001,
        disableHostCheck: true
    }
}