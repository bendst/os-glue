# os-glue

Provide obstractions for embedded OS.

On tier 1 and 2 targets the os-glue just wraps the underlying stdlib functionality
On tier 3 it is hightly dependant on the used IoT operating system

For using tier 3 you must always provide a feature flag for particular board.

## Currently Supported Tier 3 operating system

- RIOT



# [Changelog](CHANGELOG.md)

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
