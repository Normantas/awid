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

### Size comparison with other ID's
Snowflake - 64 bits (8 bytes)\
Awid - 72 bits (9 bytes)\
Xid - 96 bits (12 bytes)\
UUID - 128 bits (16 bytes)\
*Note: ID's have different use cases. A large size isn't always a bad thing.*

### Performance tests
Creating an Awid takes 50ns, and creating an Awid with a supplied timestamp takes 15ns.\
From this it seems the main bottleneck is getting the Unix time from the OS.

*These tests were done on my local machine, results may vary.*\
*To run these tests yourself, copy the Git repo and run `cargo bench`.*

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