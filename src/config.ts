interface Config {
  port: string,
}

function loadConfigFromEnv() {
  return {
    port: process.env['PORT'] || '3000',
  };
}

export {
  Config,
  loadConfigFromEnv,
};
