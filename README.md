# Booking rooms for Calgary Rust

This repository is for an app that will allow the leaders of the Calgary Rust meetup to book rooms for our events.

## Note on the phase of development

The app is currently only being developed. There's a script that showcases the future core functionality of the app but there's no actual app yet.

## Note on security

Under the hood, the app has to rely on the `spryker/chromedriver` docker image, which is [vulnerable to security issues](https://github.com/JohnScience/chromium-chromedriver). Fixing the security issues was proven to be a non-trivial task and is not done yet. The app is not meant to be used in a production environment, so the security issues are not a priority at the moment.

## Note on the web scraping and headless browser approaches

In order to get the data from the website of the Calgary Public Library, the app has to use a headless browser via the WebDriver protocol. It is currently impossible to access it in a WASM module from browser because the page has to be accessed *interactively* and parsing alone is insufficient. Theoretically, it can be possible to use a separate transparent or child WebView window but it'd require further improvement of the [`tauriless`](https://crates.io/crates/tauriless) crate.

## Running the script

```docker
docker compose build && docker compose up -d && docker attach booking-rooms-app-1
```
