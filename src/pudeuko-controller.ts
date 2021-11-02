import Router from '@koa/router';
import { Context } from 'koa';
import Logger from './logger';
import { PudeukoItemNotFoundError, PudeukoService } from './pudeuko-service';

const STATUS_OK = 200;
const STATUS_NO_CONTENT = 204;
const STATUS_NOT_FOUND = 404;
const STATUS_INTERNAL_SERVER_ERROR = 500;

interface ContentDTO {
  text: string,
}

class PudeukoController {
  private router: Router;
  private static logger = new Logger('PudeukoController');

  constructor() {
    this.router = new Router();
    this.router.prefix('/pudeuko');
    this.router.get('/', PudeukoController.getPudeuko);
    this.router.post('/', PudeukoController.addItem);
    this.router.get('/:id', PudeukoController.getItem);
    this.router.delete('/:id', PudeukoController.removeItem);
    this.router.post('/reenrich', PudeukoController.reenrichItems);
  }

  private static async getPudeuko(ctx: Context): Promise<void> {
    try {
      const pudeuko = await PudeukoService.getPudeuko();

      ctx.body = pudeuko;
      ctx.status = STATUS_OK;
    } catch (e) {
      PudeukoController.logger.withError(e);
      ctx.status = STATUS_INTERNAL_SERVER_ERROR;
    }
  }

  private static async addItem(ctx: Context): Promise<void> {
    const content: ContentDTO = ctx.request.body;

    try {
      await PudeukoService.addItemFromText(content.text);
      ctx.status = STATUS_OK;
    } catch (e) {
      PudeukoController.logger.withError(e);
      ctx.status = STATUS_INTERNAL_SERVER_ERROR;
    }
  }

  private static async getItem(ctx: Context): Promise<void> {
    const itemId = ctx.params.id;

    try {
      const item = await PudeukoService.getItem(itemId);

      ctx.body = item;
      ctx.status = STATUS_OK;
    } catch (e) {
      PudeukoController.logger.withError(e);

      if (e instanceof PudeukoItemNotFoundError) {
        ctx.status = STATUS_NOT_FOUND;
      } else {
        ctx.status = STATUS_INTERNAL_SERVER_ERROR;
      }
    }
  }

  private static async removeItem(ctx: Context): Promise<void> {
    const itemId = ctx.params.id;

    try {
      await PudeukoService.archiveItem(itemId);
      ctx.status = STATUS_NO_CONTENT;
    } catch (e) {
      PudeukoController.logger.withError(e);

      if (e instanceof PudeukoItemNotFoundError) {
        ctx.status = STATUS_NOT_FOUND;
      } else {
        ctx.status = STATUS_INTERNAL_SERVER_ERROR;
      }
    }
  }

  private static async reenrichItems(_ctx: Context): Promise<void> {
    PudeukoService.reenrichItems();
  }

  getRouter(): Router {
    return this.router;
  }
}

export default PudeukoController;
