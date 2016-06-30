"use strict";
var fs_1 = require('fs');
var hello;
try {
    hello = fs_1.readFileSync('hello.txt', 'utf8');
}
catch (err) {
    throw "Couldn't read 'hello.txt'.";
}
var world;
try {
    world = fs_1.readFileSync('world.txt', 'utf8');
}
catch (err) {
    throw "Couldn't read 'world.txt'.";
}
var helloWorld = hello + " " + world + "!";
try {
    fs_1.writeFileSync('hello-world.txt', helloWorld);
}
catch (err) {
    throw "Couldn't write 'hello-world.txt'.";
}
console.log("Wrote file in hello-world.txt with content: " + helloWorld);
//# sourceMappingURL=index.js.map