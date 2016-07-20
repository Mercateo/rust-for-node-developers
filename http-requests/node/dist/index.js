"use strict";
var https_1 = require('https');
var host = 'api.github.com';
var path = '/users/donaldpipowitch';
function isClientError(statusCode) {
    return statusCode >= 400 && statusCode < 500;
}
function isServerError(statusCode) {
    return statusCode >= 500;
}
var headers = {
    'user-agent': 'Mercateo/rust-for-node-developers'
};
https_1.get({ host: host, path: path, headers: headers }, function (res) {
    var buf = '';
    res.on('data', function (chunk) { return buf = buf + chunk; });
    res.on('end', function () {
        console.log("Response: " + buf);
        if (isClientError(res.statusCode)) {
            throw "Got client error: " + res.statusCode;
        }
        if (isServerError(res.statusCode)) {
            throw "Got server error: " + res.statusCode;
        }
    });
}).on('error', function (err) { throw "Couldn't send request."; });
//# sourceMappingURL=index.js.map