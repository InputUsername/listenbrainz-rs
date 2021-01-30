# Changelog

## v0.3.0 (unreleased)

- Add rate limiting information to response types in the `rate_limit` field;
  - See the [ListenBrainz API docs](https://listenbrainz.readthedocs.io/en/production/dev/api/#rate-limiting)
    for more information on rate limiting.

## v0.2.0 (2021-01-20)

- `Client` methods don't require `&mut self` anymore;
- Moved `Client`, `models::request` and `models::response` modules to the `raw` module;
- Added the `ListenBrainz` type, a more ergonomic and high-level API client;
- Improved overall documentation, especially for `Client`.

## v0.1.0 (2021-01-14)

Initial release.
