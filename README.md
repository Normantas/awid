# awid - Awesome ID!
A small, simple, and universally unique identifier.

## Why use (or don't use) this crate?
### Features
 - Small size of 9 bytes
 - K-ordered (explanation: nearly-sorted using timestamps)
 - Built-in timestamp with precision of a second
 - Very unlikely to have a collision (which makes it universally unique... probably)

### Collision resistance
Awid's have 5 bytes of random data, which has 2^40 possible combinations.\
That is 1,099,511,627,776 possible combinations *for each second*.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.