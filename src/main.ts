import Koa from 'koa';
import { loadConfigFromEnv } from './config';
import ItemsController from './items-controller';

const app = new Koa();
const config = loadConfigFromEnv();

const itemsController = new ItemsController();

app
  .use(itemsController.getRouter().routes())
  .use(itemsController.getRouter().allowedMethods())
  .listen(config.port, () => {
    console.log(`pudeuko started on port ${config.port}`);
  });
