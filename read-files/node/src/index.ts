import { openSync, readSync, fstatSync, Stats } from 'fs';

let fileDescriptor: number;
try {
  fileDescriptor = openSync('hello.txt', 'r');
} catch (err) {
  console.log(`Couldn't open: ${err.message}`);
  process.exit(1);
}

let stat: Stats;
try {
  stat = fstatSync(fileDescriptor);
} catch (err) {
  console.log(`Couldn't get stat: ${err.message}`);
  process.exit(1);
}

const buffer = Buffer.alloc(stat.size);

try {
  readSync(fileDescriptor, buffer, 0, stat.size, null);
} catch (err) {
  console.log(`Couldn't read: ${err.message}`);
  process.exit(1);
}

let data: string;
try {
  data = buffer.toString();
} catch (err) {
  console.log(`Couldn't convert buffer to string: ${err.message}`);
  process.exit(1);
}

console.log(`Content is: ${data}`);
