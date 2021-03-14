import Koa from 'koa';
import Router from '@koa/router';
import { loadConfigFromEnv } from './config';

const app = new Koa();
const config = loadConfigFromEnv();

const router = new Router();

router.get('/', (ctx) => {
  ctx.body = 'Hello route World';
});

app
  .use(router.routes())
  .use(router.allowedMethods())
  .listen(config.port, () => {
    console.log(`pudeuko started on port ${config.port}`);
  });
