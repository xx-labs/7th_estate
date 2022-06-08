const PROXY_CONFIG = {
    "/vote": {
        "target": "http://localhost:9098/",
        "secure": false,
        "onProxyRes": function(proxyRes, req, res) {
            delete proxyRes.headers['X-Powered-By'];
            proxyRes.headers['Access-Control-Allow-Headers'] = 'Authorization';
        },
    }
}

module.exports = PROXY_CONFIG;