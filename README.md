# Booking rooms for Calgary Rust

This repository is for an app that will allow the leaders of the Calgary Rust meetup to book rooms for our events.

## Note on the phase of development

The app is currently only being developed. There's a script that showcases the future core functionality of the app but there's no actual app yet.

## Note on security

Under the hood, the app has to rely on the `spryker/chromedriver` docker image, which is [vulnerable to security issues](https://github.com/JohnScience/chromium-chromedriver). Fixing the security issues was proven to be a non-trivial task and is not done yet. The app is not meant to be used in a production environment, so the security issues are not a priority at the moment.
