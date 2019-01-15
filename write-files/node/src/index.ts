import { readFileSync, writeFileSync } from 'fs';

let hello: string;
try {
  hello = readFileSync('hello.txt', 'utf8');
} catch (err) {
  throw `Couldn't read 'hello.txt'.`;
}

let world: string;
try {
  world = readFileSync('world.txt', 'utf8');
} catch (err) {
  throw `Couldn't read 'world.txt'.`;
}

let helloWorld = `${hello} ${world}!`;

try {
  writeFileSync('hello-world.txt', helloWorld);
} catch (err) {
  throw `Couldn't write 'hello-world.txt'.`;
}

console.log(`Wrote file 'hello-world.txt' with content: ${helloWorld}`);
