import fetch, { Response } from 'node-fetch';
import { loadConfigFromEnv } from './config';
import { PudeukoObject } from './model';

const DROPBOX_FILE_PATH = '/pudeuko/data.json';
const DROPBOX_DOWNLOAD_URL = 'https://content.dropboxapi.com/2/files/download';
const DROPBOX_UPLOAD_URL = 'https://content.dropboxapi.com/2/files/upload';

enum DropboxRequestType {
  DOWNLOAD,
  UPLOAD,
}

class DropboxStorage {
  static async read(): Promise<PudeukoObject> {
    const response = await DropboxStorage.request(DropboxRequestType.DOWNLOAD);
    const json = await response.json();
    return json;
  }

  static async write(pudeuko: PudeukoObject): Promise<void> {
    await DropboxStorage.request(DropboxRequestType.UPLOAD, pudeuko);
  }

  private static async request(type: DropboxRequestType, pudeuko?: PudeukoObject): Promise<Response> {
    const token = loadConfigFromEnv().dropboxToken;
    const url = (type === DropboxRequestType.DOWNLOAD) ? DROPBOX_DOWNLOAD_URL : DROPBOX_UPLOAD_URL;
    const dropboxApiArg = (type === DropboxRequestType.DOWNLOAD)
      ? { path: DROPBOX_FILE_PATH }
      : { path: DROPBOX_FILE_PATH, mode: 'overwrite' };
    const body = (type === DropboxRequestType.UPLOAD)
      ? JSON.stringify(pudeuko)
      : undefined;

    const options = {
      method: type === DropboxRequestType.DOWNLOAD ? 'GET' : 'POST',
      body,
      headers: {
        Authorization: `Bearer ${token}`,
        'Dropbox-API-Arg': JSON.stringify(dropboxApiArg),
        ...(DropboxRequestType.UPLOAD ? { 'Content-Type': 'application/octet-stream' } : {}),
      },
    };

    const response = await fetch(url, options);

    if (response.ok === false) {
      throw new Error(`Request failed, HTTP status ${response.status}`);
    }

    return response;
  }
}

export default DropboxStorage;
