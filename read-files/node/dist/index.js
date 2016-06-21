"use strict";
var fs_1 = require('fs');
var file;
try {
    file = fs_1.openSync('hello.txt', 'r');
}
catch (err) {
    console.log("Couldn't open: " + err.message);
    process.exit(1);
}
var stat;
try {
    stat = fs_1.fstatSync(file);
}
catch (err) {
    console.log("Couldn't get stat: " + err.message);
    process.exit(1);
}
var buffer = new Buffer(stat.size);
try {
    fs_1.readSync(file, buffer, 0, stat.size, null);
}
catch (err) {
    console.log("Couldn't read: " + err.message);
    process.exit(1);
}
var data;
try {
    data = buffer.toString();
}
catch (err) {
    console.log("Couldn't convert buffer to string: " + err.message);
    process.exit(1);
}
console.log("Content is: " + data);
//# sourceMappingURL=index.js.map