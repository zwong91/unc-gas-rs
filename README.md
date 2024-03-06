# unc-gas

<p>
    <a href="https://crates.io/crates/unc-gas"><img src="https://img.shields.io/crates/d/unc-gas?style=flat-square&logo=near&label=crates.io" alt="Crates.io (downloads)"></a>
    <a href="https://docs.rs/unc-gas/latest/unc_gas/"><img src="https://img.shields.io/docsrs/unc-gas?style=flat-square" alt="Docs.rs"></a>
    <img src="https://img.shields.io/badge/rustc-1.68%2B-lightgray.svg?style=flat-square" alt="Rust Version">
</p>

unc-gas is crate to ergonomically operate with NEAR Protocol gas unit in Rust projects.

The crate includes UncGas type and constructors for converting data as UncGas and as u64 type values.

## unc-gas examples 

```rust
use unc_gas::UncGas;

fn main() {
    let data = "12.657 tgas";

    let unc_gas: UncGas = data.parse().unwrap();

    // Convert the value to the most precise "gas" unit
    assert_eq!(unc_gas.as_gas(), 12657000000000);
    // Convert the value to "gigagas" unit
    assert_eq!(unc_gas.as_ggas(), 12657);
    
    // Display Gas. It will print: "Here is 12.7 Tgas"
    println!("Here is {}", unc_gas);

    // When `serde` feature is enabled, UncGas can be used in serde-serializable structs.
    // UncGas will be serialized to a gas-precision u64 value encoded as string.
    #[derive(serde::Serialize)]
    struct FunctionCallDetails {
        used_gas: UncGas,
    }

    let details = FunctionCallDetails { used_gas: unc_gas };

    assert_eq!(serde_json::to_string(&details).unwrap(), r#"{"used_gas":"12657000000000"}"#);
}
```

## UncGas information

On every transaction you send to the network NEAR charges you a fee (aka gas fee). This fee is used to indirectly pay the people that keep the network infrastructure, and to incentivize developers of smart contracts. [For more information].

[Gas usage in Near Protocol]

## Crate Features

* `serde` - [serde](https://serde.rs/) support
* `borsh` - [borsh](https://github.com/near/borsh-rs) support
* `abi` - [unc-abi](https://github.com/near/abi) support
* `schemars` - [schemars](https://github.com/GREsau/schemars) support
* `interactive-clap` - [interactive-clap](https://github.com/unc-cli-rs/interactive-clap) support

### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/Mr0melian/unc_gas/blob/master/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/Mr0melian/unc_gas/blob/master/LICENSE-APACHE
[For more information]: https://docs.near.org/concepts/basics/transactions/gas
[Gas usage in Near Protocol]: https://nomicon.io/RuntimeSpec/Fees/
