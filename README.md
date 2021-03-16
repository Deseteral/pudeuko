# pudeuko
![Build Status](https://github.com/Deseteral/pudeuko/workflows/Build/badge.svg)

REST service for storing internet things (or... links 🤔) you wish to check out later.

# Development
```sh
npm install
npm run build
npm start # to run service
npm test  # to run tests
```

## Endpoints
`GET /pudeuko` \
Returns your pudeuko object containing your saved items (in `item` array) and archived ones (in `archive` array).

`POST /pudeuko` \
Accepts `application/json` \
Example body: `{ "text": "Check out this link https://example.com/cool_content" }` \
Parses string content containing link to save. That link is being transformed into an item and saved to data storage.

`GET /pudeuko/<id>` \
Returns an item for given `id`.

`DELETE /pudeuko/<id>` \
Archives an item if it is found by given `id`.

## Data storage
`pudeuko` uses Dropbox as it's data storage. To run the app you have to provide Dropbox app token via `DROPBOX_TOKEN`
environment variable.

The data is saved in a single JSON file located at `/pudeuko/data.json`.

## License
This project is licensed under the [MIT license](LICENSE).
