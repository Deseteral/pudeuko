import DropboxStorage from './dropbox-storage';
import { PudeukoObject } from './model';

class PudeukoService {
  static async getPudeuko(): Promise<PudeukoObject> {
    return DropboxStorage.read();
  }
}

export {
  PudeukoService,
};
