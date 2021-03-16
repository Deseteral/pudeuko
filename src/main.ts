import Koa from 'koa';
import bodyParser from 'koa-bodyparser';
import { loadConfigFromEnv } from './config';
import Logger from './logger';
import PudeukoController from './pudeuko-controller';

const logger = new Logger('Main');
const app = new Koa();
const config = loadConfigFromEnv();

const pudeukoController = new PudeukoController();

app
  .use(bodyParser())
  .use(pudeukoController.getRouter().routes())
  .use(pudeukoController.getRouter().allowedMethods())
  .listen(config.port, () => {
    logger.info(`pudeuko started on port ${config.port}`);
  });
