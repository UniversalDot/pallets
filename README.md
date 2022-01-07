# Pallets
Custom pallets developed by UniversalDot Foundation


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
