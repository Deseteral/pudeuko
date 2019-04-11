# pudeuko
[![Build Status](https://travis-ci.org/Deseteral/pudeuko.svg?branch=master)](https://travis-ci.org/Deseteral/pudeuko)

REST service for storing internet things (or... links 🤔) you wish to check out later.

*Written in Rust using actix-web.*

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

`GET /items/<id>` \
Returns an item for given `id`

## Data storage
`pudeuko` uses Dropbox as it's data storage. To run the app you have to provide Dropbox app token
via  `DROPBOX_TOKEN` environment variable.

The data is saved in a single JSON file located at `/pudeuko/data.json`.

## Heroku Deployment
To deploy this app on Heroku you have to set a Rust buildpack:
```sh
heroku apps:create
heroku buildpacks:set emk/rust
git push heroku master
```

## License
This project is licensed under the [MIT license](LICENSE).
