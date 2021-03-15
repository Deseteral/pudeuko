import getUrls from 'get-urls';
import { nanoid } from 'nanoid';
import DropboxStorage from './dropbox-storage';
import { PudeukoItem, PudeukoLink, PudeukoObject } from './model';

class PudeukoItemNotFoundError extends Error {
  constructor(itemId: string) {
    super(`Pudeuko does not contain item with id: ${itemId}`);
    this.name = 'PudeukoItemNotFoundError';
  }
}

class PudeukoService {
  static async getPudeuko(): Promise<PudeukoObject> {
    return DropboxStorage.read();
  }

  static async getItem(itemId: string): Promise<PudeukoItem> {
    const pudeuko = await DropboxStorage.read();
    const item = pudeuko.items.find((it) => it.id === itemId);

    if (!item) throw new PudeukoItemNotFoundError(itemId);

    return item;
  }

  static async addItemFromText(text: string): Promise<void> {
    const item = this.simpleItemFromText(text);

    const pudeuko = await DropboxStorage.read();
    pudeuko.items.unshift(item);
    await DropboxStorage.write(pudeuko);
  }

  static async removeItem(itemId: string): Promise<void> {
    const pudeuko = await DropboxStorage.read();
    const index = pudeuko.items.findIndex((item) => item.id === itemId);

    if (index === -1) throw new PudeukoItemNotFoundError(itemId);

    pudeuko.items.splice(index, 1);
    await DropboxStorage.write(pudeuko);
  }

  private static simpleItemFromText(text: string): PudeukoItem {
    const url: (string | undefined) = getUrls(text).values().next().value;
    const link: (PudeukoLink | undefined) = url ? { url } : undefined;

    return {
      id: nanoid(),
      text,
      link,
      createdAt: new Date(),
    };
  }
}

export {
  PudeukoService,
  PudeukoItemNotFoundError,
};
