# ethabi-decode

This library decodes ABI-encoded data and event logs. It is a fork of [ethabi](https://github.com/openethereum/ethabi) with a focus on providing decode functionality in environments where `libstd` may not be available.

For compatibility with constrained `no_std` environments, the design of this library differs from the the upstream [ethabi](https://github.com/openethereum/ethabi) in several respects:
* ABI's need to be specified as code rather than being loaded from JSON.
* Use of `Vec<u8>` instead of `std::string::String` for owned strings.
* All primitives for dealing with human-readable error and display output were excised.


## Building

- Build without `libstd`

  ```
  cargo build --no-default-features
  ```

- Build with `libstd`

  ```
  cargo build
  ```

