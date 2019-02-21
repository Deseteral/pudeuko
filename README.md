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

## Data storage
`pudeuko` uses Dropbox as it's data storage. To run the app you have to provide Dropbox app token
via  `DROPBOX_TOKEN` environment variable.

The data is saved in a single JSON file located at `/pudeuko/data.json`.

## License
This project is licensed under the [MIT license](LICENSE).
