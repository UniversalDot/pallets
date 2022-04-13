# Pallets
This is a playground for new pallets that are developed for Substrate. 

The pallets used in production have been moved to the [node](https://github.com/UniversalDot/universal-dot-node). 

Pallets are developed separately from [Runtime](https://github.com/UniversalDot/universal-dot-node) and injected into the Runtime. 

Custom Pallet cargo versions shall remain consistent with Runtime package versions.

## Development
To build each pallet, run: 

```bash
cargo build
```

Optionally, you can install clippy which is a rust tool that check for Rust to improve common mistakes. To install clippy: https://github.com/rust-lang/rust-clippy

To run clippy locally, run: 

```bash
cargo clippy
```

## Add Pallet to Runtime
To add a specific pallet to a Runtime, navigate to the [Universal-dot-node](https://github.com/UniversalDot/universal-dot-node). In the /Runtime/Cargo.toml add each pallet dependency. For example, to add the Profile pallet, the following configuration should be added:

```bash
[dependencies.pallet-profile]
default-features = false
git = 'https://github.com/UniversalDot/pallets.git'
version = '0.0.67'
```

## Testing

##### Mock Runtime

Pallet test depend on substrate Runtime. To be able to run these pallet test, first we must construct a mock Runtime environment. Mocks for runtime are constructed for each pallet separately and are located in their respective /src/ folder. <br>
More information regarding constructing Mock Runtime at the following [link](https://docs.substrate.io/v3/runtime/testing/#mock-runtime-environment).

##### Writing tests

Tests are functions, annotated with the #[test] macro. To test specific functionality, we use <b>assert</b> macros to match our expectations with the expected result. There are already several predefine assert macros such as:
* assert_eq!
* assert_ok!
* assert_noop!

For more information on how to create tests, refer to the following [link](https://docs.substrate.io/how-to-guides/v3/testing/basics/). 

##### Run tests

To run test each pallet, simply run at pallet root: 

```bash
cargo test
```

To run benchmark tests for each pallet, simply run at pallet root: 

```bash
cargo test --features runtime-benchmarks
```


![Logo](https://github.com/UniversalDot/documents/blob/master/logo/rsz_jpg-02.jpg)
