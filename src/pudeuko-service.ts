import cheerio from 'cheerio';
import getUrls from 'get-urls';
import { nanoid } from 'nanoid';
import fetch from 'node-fetch';
import { URL } from 'url';
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

    console.log(`Added new pudeuko item with id ${item.id}`);

    try {
      this.enrichItem(item.id);
    } catch (e) {
      console.error(e);
    }
  }

  static async removeItem(itemId: string): Promise<void> {
    const pudeuko = await DropboxStorage.read();
    const index = pudeuko.items.findIndex((item) => item.id === itemId);

    if (index === -1) throw new PudeukoItemNotFoundError(itemId);

    pudeuko.items.splice(index, 1);
    await DropboxStorage.write(pudeuko);

    console.log(`Removed pudeuko item with id ${itemId}`);
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

  private static async enrichItem(itemId: string): Promise<void> {
    const pudeuko = await this.getPudeuko();
    const index = pudeuko.items.findIndex((it) => it.id === itemId);

    if (index === -1) return;

    const item = pudeuko.items[index];

    if (!item.link) return;
    if (!item.text.startsWith('http') || item.text.split(' ').length > 1) return;

    const response = await fetch(item.link.url);
    if (!response.ok) return;

    const html = await response.text();
    const $ = cheerio.load(html);

    item.text = $('title').text();
    item.icon = {
      src: `${new URL(item.link.url).origin}/favicon.ico`,
    };

    pudeuko.items[index] = item;
    DropboxStorage.write(pudeuko);

    console.log(`Enriched data for pudeuko item with id ${itemId}`);
  }
}

export {
  PudeukoService,
  PudeukoItemNotFoundError,
};
