import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch';

function isClientError(statusCode: number) {
  return statusCode >= 400 && statusCode < 500;
}

function isServerError(statusCode: number) {
  return statusCode >= 500;
}

const headers = {
  'User-Agent': 'Mercateo/rust-for-node-developers'
};

get({ host, path, headers }, (res) => {
  let buf = '';
  res.on('data', (chunk) => (buf = buf + chunk));

  res.on('end', () => {
    console.log(`Response: ${buf}`);

    if (isClientError(res.statusCode)) {
      throw `Got client error: ${res.statusCode}`;
    }
    if (isServerError(res.statusCode)) {
      throw `Got server error: ${res.statusCode}`;
    }
  });
}).on('error', (err) => {
  throw `Couldn't send request.`;
});
