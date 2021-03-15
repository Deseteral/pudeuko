import { nanoid } from 'nanoid';
import DropboxStorage from './dropbox-storage';
import { PudeukoItem, PudeukoObject } from './model';

class PudeukoService {
  static async getPudeuko(): Promise<PudeukoObject> {
    return DropboxStorage.read();
  }

  static async addItemFromText(text: string): Promise<void> {
    const item = this.mapTextToItem(text);

    const pudeuko = await DropboxStorage.read();
    pudeuko.items.unshift(item);
    await DropboxStorage.write(pudeuko);
  }

  static mapTextToItem(text: string): PudeukoItem {
    return {
      id: nanoid(),
      createdAt: new Date(),
      text,
    };
  }
}

export {
  PudeukoService,
};
