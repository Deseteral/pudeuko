import fetch from 'node-fetch';
import { loadConfigFromEnv } from './config';
import { PudeukoItem } from './model';

const DROPBOX_FILE_PATH = '/pudeuko/data.json';
const DROPBOX_DOWNLOAD_URL = 'https://content.dropboxapi.com/2/files/download';
const DROPBOX_UPLOAD_URL = 'https://content.dropboxapi.com/2/files/upload';

class DropboxStorage {
  static async read(): Promise<PudeukoItem[]> {
    const token = loadConfigFromEnv().dropboxToken;

    const data = await fetch(DROPBOX_DOWNLOAD_URL, {
      headers: {
        Authorization: `Bearer ${token}`,
        'Dropbox-API-Arg': JSON.stringify({ path: DROPBOX_FILE_PATH }),
      },
    });

    const json = await data.json();
    return json;
  }

  static async write(list: PudeukoItem[]): Promise<void> {
    const token = loadConfigFromEnv().dropboxToken;

    await fetch(DROPBOX_UPLOAD_URL, {
      method: 'POST',
      body: JSON.stringify(list),
      headers: {
        Authorization: `Bearer ${token}`,
        'Dropbox-API-Arg': JSON.stringify({ path: DROPBOX_FILE_PATH, mode: 'overwrite' }),
      },
    });
  }
}

export default DropboxStorage;

// impl DropboxStorage {
//     pub fn new(dropbox_token: &str) -> Self {
//         let mut default_headers = HeaderMap::new();
//         default_headers.insert(
//             AUTHORIZATION,
//             format!("Bearer {}", &dropbox_token).parse().unwrap(),
//         );

//         let mut download_headers = HeaderMap::new();
//         download_headers.insert(
//             "Dropbox-API-Arg",
//             json!({ "path": DROPBOX_FILE_PATH })
//                 .to_string()
//                 .parse()
//                 .unwrap(),
//         );

//         let mut upload_headers = HeaderMap::new();
//         upload_headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
//         upload_headers.insert(
//             "Dropbox-API-Arg",
//             json!({ "path": DROPBOX_FILE_PATH, "mode": "overwrite" })
//                 .to_string()
//                 .parse()
//                 .unwrap(),
//         );

//         Self {
//             client: ClientBuilder::new()
//                 .default_headers(default_headers)
//                 .build()
//                 .unwrap(),
//             download_headers,
//             upload_headers,
//         }
//     }
// }

// impl Storage for DropboxStorage {
//     fn read(&self) -> ItemList {
//         let body = self
//             .client
//             .get(DROPBOX_DOWNLOAD_URL)
//             .headers(self.download_headers.clone())
//             .send()
//             .unwrap()
//             .text()
//             .unwrap();

//         let items: ItemList = serde_json::from_str(&body).unwrap();
//         items
//     }

//     fn write(&mut self, list: ItemList) {
//         let json = serde_json::to_string(&list).unwrap();

//         self.client
//             .post(DROPBOX_UPLOAD_URL)
//             .headers(self.upload_headers.clone())
//             .body(json)
//             .send()
//             .unwrap();
//     }
// }
