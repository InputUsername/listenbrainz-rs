# Changelog

## v0.7.0 (2023-02-12)

- Raw request types are now generic over their string types ([#14], [@mgziminsky]):
  - **This is a breaking change.**
  - Added the trait `StrType` which is implemented for all types that implement `Borrow<str> + Serialize`;
  - Changed `SubmitListens<'a>` to `SubmitListens<'a, Track: StrType, Artist: StrType, Release: StrType>`;
  - Changed `Payload<'a>` to `Payload<Track: StrType, Artist: StrType, Release: StrType>`;
  - Changed `TrackMetadata<'a>` to `TrackMetadata<Track: StrType, Artist: StrType, Release: StrType>`;
  - Added the `Empty` type, which can be used to disambiguate the type of `TrackMetadata::release`;
    - If the `release` field is `None`, use `None::<Empty>`.
  - Changed `DeleteListen<'a>` to `DeleteListen<T: StrType>`;
  - Changed `Client::submit_listens` to `Client::submit_listens<Track: StrType, Artist: StrType, Release: StrType>`;
  - Changed `Client::delete_listen` to `Client::delete_listen<T: StrType>`;
  - The `ListenBrainz` API remains unchanged.
- Changed `Client::new_with_url(url: &str)` to `Client::new_with_url(url: impl ToString)`.

[#14]: https://github.com/InputUsername/listenbrainz-rs/pull/14
[@mgziminsky]: https://github.com/mgziminsky

## v0.6.0 (2023-01-26)

- Updated response schemas to match the ListenBrainz API ([#12], [#13], [@cellularnetwork]):
  - **These are breaking changes.**
  - `user/{user_name}/playing-now`:
    - Added `UserPlayingNowPayload::playing_now`;
    - Added `UserPlayingNowListen::playing_now`;
    - Removed `UserPlayingNowListen::{user_name, inserted_at, recording_msid}.
  - `user/{user_name}/listens`:
    - Changed the type of `UserListensListen::inserted_at` from `String` to `i64`.
  - `stats/sitewide/artists`:
    - Removed `StatsSitewideArtistsPayload::time_ranges`;
    - Added `StatsSitewideArtistsPayload::artists`;
    - Removed `StatsSitewideArtistsTimeRange`;
    - Removed `StatsSitewideArtistsArtist::artist_msid`.
  - `stats/user/{user_name}/artist-map`:
    - Changed the return type of `Client::stats_user_artist_map` from `Result<StatsUserArtistMapResponse, Error>`
      to `Result<Option<StatsUserArtistMapResponse>, Error>`.
  - `users/{user_list}/recent-listens`:
    - Removed `Client::users_recent_listens`;
    - Removed `UsersRecentListensResponse`;
    - Removed `UsersRecentListensPayload`;
    - Removed `UsersRecentListensListen`;
    - Removed `UsersRecentListensTrackMetadata`;
    - Removed `examples/users_recent_listens.rs`.

[#12]: https://github.com/InputUsername/listenbrainz-rs/pull/12
[#13]: https://github.com/InputUsername/listenbrainz-rs/pull/13
[@cellularnetwork]: https://github.com/cellularnetwork

## v0.5.0 (2022-12-05)

- Made the `release` parameter of `ListenBrainz` methods optional ([#11], [@mgziminsky]).
  - **This is a breaking change.**

[#11]: https://github.com/InputUsername/listenbrainz-rs/pull/11
[@mgziminsky]: https://github.com/mgziminsky

## v0.4.3 (2022-11-16)

- Updated attohttpc dependency.

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
