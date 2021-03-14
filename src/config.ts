interface Config {
  port: string,
  dropboxToken: string,
}

function loadConfigFromEnv(): Config {
  const dropboxToken = process.env['DROPBOX_TOKEN'];

  if (!dropboxToken) {
    throw new Error('Environment variable DROPBOX_TOKEN is required');
  }

  return {
    port: process.env['PORT'] || '3000',
    dropboxToken,
  };
}

export {
  Config,
  loadConfigFromEnv,
};
