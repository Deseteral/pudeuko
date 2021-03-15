import Koa from 'koa';
import bodyParser from 'koa-bodyparser';
import { loadConfigFromEnv } from './config';
import PudeukoController from './pudeuko-controller';

const app = new Koa();
const config = loadConfigFromEnv();

const pudeukoController = new PudeukoController();

app
  .use(bodyParser())
  .use(pudeukoController.getRouter().routes())
  .use(pudeukoController.getRouter().allowedMethods())
  .listen(config.port, () => {
    console.log(`pudeuko started on port ${config.port}`);
  });
