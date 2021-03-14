import DropboxStorage from './dropbox-storage';
import { PudeukoItem } from './model';

class PudeukoService {
  static async getItems(): Promise<PudeukoItem[]> {
    return DropboxStorage.read();
  }
}

export {
  PudeukoService,
};
