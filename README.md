# pudeuko
[![Build Status](https://travis-ci.org/Deseteral/pudeuko.svg?branch=master)](https://travis-ci.org/Deseteral/pudeuko)

REST service for storing quick links you wish to check out later.

*Written in Rust using Rocket framework.*

## Development
To compile you have to install Rust nightly.

Use Cargo to compile and run tests:
```sh
cargo run
cargo test
```

## Endpoints

`GET /items` \
Returns array of items

`POST /items` \
Accepts `application/json` \
Example body: `{ "text": "Check out this link https://example.com/cool_content" }` \
Parses string content containing link to save. That link is being transformed into an item and saved
to data storage.

## Data storage
`pudeuko` uses Dropbox as it's data storage. To run the app you have to provide Dropbox app token
via  `DROPBOX_TOKEN` environment variable.

The data is saved in a single JSON file located at `/pudeuko/data.json`.

## License
This project is licensed under the [MIT license](LICENSE).
