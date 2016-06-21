import { openSync, readSync, fstatSync } from 'fs';

let file;
try {
  file = openSync('hello.txt', 'r');
} catch (err) {
  console.log(`Couldn't open: ${err.message}`);
  process.exit(1);
}

let stat;
try {
  stat = fstatSync(file);
} catch (err) {
  console.log(`Couldn't get stat: ${err.message}`);
  process.exit(1);
}

const buffer = new Buffer(stat.size);

try {
  readSync(file, buffer, 0, stat.size, null);
} catch (err) {
  console.log(`Couldn't read: ${err.message}`);
  process.exit(1);
}

let data;
try {
  data = buffer.toString();
} catch (err) {
  console.log(`Couldn't convert buffer to string: ${err.message}`);
  process.exit(1);
}

console.log(`Content is: ${data}`);