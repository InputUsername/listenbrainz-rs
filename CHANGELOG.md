# Changelog

## v0.4.2 (2022-05-12)

- Added the `ListenBrainz::import()` method to allow importing
  listens at a specified timestamp.

## v0.4.1 (2021-12-03)

- Fixed a broken link in the documentation;
- Added the `api_url()` method to `raw::Client` and `ListenBrainz`;
- Implemented `Default` for `raw::Client` and `ListenBrainz`;
  - Returns the same as `Client::new()`/`ListenBrainz::new()`.

## v0.4.0 (2021-10-21)

- Added CI builds support ([#1]);
- Added support for alternative ListenBrainz hosts ([#6]);
- Changed from [`ureq`] to [`attohttpc`] for performing HTTP requests ([#7]).

[#1]: https://github.com/InputUsername/listenbrainz-rs/pull/1
[#6]: https://github.com/InputUsername/listenbrainz-rs/pull/6
[#7]: https://github.com/InputUsername/listenbrainz-rs/pull/7

## v0.3.0 (2021-02-02)

- Added rate limiting information to response types in the `rate_limit` field;
  - See the [ListenBrainz API docs](https://listenbrainz.readthedocs.io/en/production/dev/api/#rate-limiting)
    for more information on rate limiting.

## v0.2.0 (2021-01-20)

- `Client` methods don't require `&mut self` anymore;
- Moved `Client`, `models::request` and `models::response` modules to the `raw` module;
- Added the `ListenBrainz` type, a more ergonomic and high-level API client;
- Improved overall documentation, especially for `Client`.

## v0.1.0 (2021-01-14)

Initial release.
