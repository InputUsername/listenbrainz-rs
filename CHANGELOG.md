# Changelog

## v0.8.0 (unreleased)

- Implemented additional API endpoints:
  - Core:
    - `/1/user/(user_name)/similar-users` - `Client::user_similar_users`;
    - `/1/user/(user_name)/similar-to/(other_user_name)` - `Client::user_similar_to`.
  - Playlists:
    - `/1/user/(playlist_user_name)/playlists` - `Client::user_playlists`;
    - `/1/user/(playlist_user_name)/playlists/createdfor` - `Client::user_playlists_created_for`;
    - `/1/user/(playlist_user_name)/playlists/collaborator` - `Client::user_playlists_collaborator`;
    - `/1/playlist/(playlist_mbid)` - `Client::get_playlist` ([#19], [@Kernald]).
    - `/1/playlist/create` - `Client::playlist_create`;
    - `/1/playlist/(playlist_mbid)/delete` - `Client::playlist_delete`;
    - `/1/playlist/(playlist_mbid)/copy` - `Client::playlist_copy`;
  - Statistics:
    - `/1/stats/release-group/(release_group_mbid)/listeners` - `Client::release_group_listeners` ([#23], [@RustyNova016]).
  - Social:
    - `/1/user/(user_name)/followers` - `Client::user_followers`;
    - `/1/user/(user_name)/following` - `Client::user_following`;
    - `/1/user/(user_name)/follow` - `Client::user_follow`;
    - `/1/user/(user_name)/unfollow` - `Client::user_unfollow`.
- Added types for playlists in MusicBrainz's [JSPF format].
- Added MBID mapping to `UserListensResponse` models ([#24], [@RustyNova016]).
- Added `Clone`, `PartialEq` and `Eq` derives for `raw` models ([#26], [@RustyNova016]).
- Removed the `time_range` parameter from `Client::user_listens` ([#24], [@RustyNova016]).
- Updated attohttpc dependency from 0.24 to 0.28.
- Pinned the minimum supported Rust version (MSRV) to 1.58.

[JSPF format]: https://musicbrainz.org/doc/jspf
[#19]: https://github.com/InputUsername/listenbrainz-rs/pull/19
[#23]: https://github.com/InputUsername/listenbrainz-rs/pull/23
[#24]: https://github.com/InputUsername/listenbrainz-rs/pull/24
[#26]: https://github.com/InputUsername/listenbrainz-rs/pull/26
[@Kernald]: https://github.com/Kernald
[@RustyNova016]: https://github.com/RustyNova016

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
