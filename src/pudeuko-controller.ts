import Router from '@koa/router';
import { Context } from 'koa';
import { PudeukoService } from './pudeuko-service';

interface ContentDTO {
  text: string,
}

class PudeukoController {
  private router: Router;

  constructor() {
    this.router = new Router();
    this.router.prefix('/pudeuko');
    this.router.get('/', PudeukoController.getPudeuko);
    this.router.post('/', PudeukoController.addItem);
  }

  private static async getPudeuko(ctx: Context): Promise<void> {
    try {
      ctx.body = await PudeukoService.getPudeuko();
      ctx.status = 200;
    } catch (e) {
      console.error(e);
      ctx.status = 500;
    }
  }

  private static async addItem(ctx: Context): Promise<void> {
    const content: ContentDTO = ctx.request.body;

    ctx.body = '';

    try {
      await PudeukoService.addItemFromText(content.text);
      ctx.status = 200;
    } catch (e) {
      console.error(e);
      ctx.status = 500;
    }
  }

  getRouter(): Router {
    return this.router;
  }
}

export default PudeukoController;

// pub fn get_items(shared_service: web::Data<SharedPudeukoService>) -> HttpResponse {
//     let service = shared_service.lock().unwrap();
//     let list = service.get_all();

//     HttpResponse::Ok().json(list)
// }

// pub fn post_item(
//     content: web::Json<ContentDTO>,
//     shared_service: web::Data<SharedPudeukoService>,
// ) -> HttpResponse {
//     let mut service = shared_service.lock().unwrap();
//     let item = Item::from(content.0);

//     service.add_item(item.clone());
//     HttpResponse::Ok().json(item)
// }

// pub fn get_item(
//     path: web::Path<(String,)>,
//     shared_service: web::Data<SharedPudeukoService>,
// ) -> HttpResponse {
//     let service = shared_service.lock().unwrap();
//     let id = &path.0;

//     match service.get_item_by_id(&id) {
//         Some(item) => HttpResponse::Ok().json(item),
//         None => HttpResponse::NotFound().body(format!("Item with id '{}' was not found", &id)),
//     }
// }

// pub fn delete_item(
//     path: web::Path<(String,)>,
//     shared_service: web::Data<SharedPudeukoService>,
// ) -> HttpResponse {
//     let mut service = shared_service.lock().unwrap();
//     let id = &path.0;

//     if service.get_item_by_id(&id).is_some() {
//         service.remove_item_by_id(&id);
//         HttpResponse::NoContent().body(format!("Item with id '{}' was removed", &id))
//     } else {
//         HttpResponse::NotFound().body(format!("Item with id '{}' was not found", &id))
//     }
// }
