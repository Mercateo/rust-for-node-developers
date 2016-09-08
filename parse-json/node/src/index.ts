import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch/repos';

function isClientError(statusCode) {
  return statusCode >= 400 && statusCode < 500;
}

function isServerError(statusCode) {
  return statusCode >= 500;
}

const headers = {
  'user-agent': 'Mercateo/rust-for-node-developers'
};

interface Repository {
  name: string;
  description: string;
  fork: boolean;
}

get({ host, path, headers }, (res) => {
  let buf = '';
  res.on('data', (chunk) => buf = buf + chunk);

  res.on('end', () => {
    if (isClientError(res.statusCode)) {
      throw `Got client error: ${res.statusCode}`
    }
    if (isServerError(res.statusCode)) {
      throw `Got server error: ${res.statusCode}`
    }

    const repositories: Array<Repository> = JSON.parse(buf)
      .map(({ name, description, fork }) => ({ name, description, fork }));
    console.log(`Result is:\n`, repositories);
  });
}).on('error', (err) => { throw `Couldn't send request.` });
