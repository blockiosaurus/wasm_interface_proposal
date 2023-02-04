# Solana Program with a WASM API and Zero Copy Deserialization
## Problem Statement
Currently, Solana programs require the implementation of off-chain client APIs in order to interact with, (de)serialize, and process on-chain programs and data. This effectively requires that all client code written in a language other than the one the on-chain program is written in duplicate all data types or functions in order to achieve feature parity with the on-chain code. As the most common client language is Javascript or Typescript, this often necessitates doubling amount of code and effort applied to writing an on-chain program; once for the on-chain code, once for the off-chain client.

Off chain data retrieval also almost entirely relies on strict deserialization formats, resulting in rigidly defined data structures that are impossible to port between different on-chain implementations of a common interface. Generic interfaces can't be defined when there's no separation between the program interface and its underlying data. Writing a plugin architecture to variably implement different data fields for different, but similar, programs with different feature implementations is non-trivial for most engineers. Discovery also becomes very difficult with this type of method.

## Proposed Solution
The solution this repository proposes follows a Write Once, Run Anywhere (WORA) philosophy where generic interface code is written alongside the on-chain program itself. Using WASM bindings and interfaces built as WASM packages, it's possible to build the actual API code in such a way that it can be run on any platforms which support the WASM runtime. The Rust code used within the on-chain program can then be cross-compiled be used off-chain as well.

### Abstracted Data Reads
It's almost universally accepted that accessors and mutators are the best practice for defining a data retrieval interface. Getters/Setters encapsulate underlying data formats and functionality, allowing for modifications and backwards compatibility so long as the interface does not change. Currently getter/setter interface design is extremely difficult to do using Solana standard practices.

The current standard is for Solana clients to pull the serialized account data from on-chain (usually Borsh serialized data) and deserialize it to a type written in the client language (e.g. Javascript). This not only necessitates porting the SerDes lib (Borsh, ugh) to client languages before it's possible to write a Solana client, but also strictly ties the on-chain data layout to the client code. There's no room for varied implementations of interfaces without the data format changing between implementations.

Enter WASM-compiled Accessors/Mutators. The getters and setters written and used by the on-chain Rust code can be cross-compiled to WASM packages that can then be run anywhere. Data reads and data layouts can be abstracted away by the interface functions and implementation details can vary so long as the core interface is followed. These can be done via a published and agreed upon Rust Trait or via a agreed set of Static functions.

### Separate Read/Write and Serialize/Deserialize paths
Another benefit that arises from abstracted data read functions is the ability to separate serialization schemas from the interface. As stated above, with the current standard practice any SerDes library used for on-chain account data must also be ported to any language that a developer desires to write a client in. This places a large constraint on both client language options and serialization formats that can be used, oftentimes resulting in compromises that limit accessibility of Solana programming and performance of on-chain programs. This repository uses [rkyv](https://rkyv.org/), a promising SerDes library that places emphasis on zero-copy serialization and deserialization at the cost of no portability to other languages aside from Rust.

### Program-Defined Instructions (maybe?)
While the above sections outline a proposal for reading on-chain data via an abstracted interface, it leaves out the hugely important write pathway for sending data from a client to the chain. This is an especially complex problem on Solana where accounts need to be declared when they're sent to the runtime. A method for resolving accounts must therefore be devised so that on-chain programs can represent the same interface even if they have differing account structures.

One promising option that is currently unexplored by this repository is the idea of including instruction builders directly in the on-chain program code. This is already a standard practice for most programs that allow CPI or Rust clients. The next steps are to compile those functions that return Instructions to WASM and change the return to a data type compatible with all client code. Returning a universal Instruction could be accomplished by allowing signature and compilation of the Transaction directly in the WASM package with the returned data being the hash that can be sent directly to an RPC node.

## Caveats/Unknowns
### WASM is still in rapid development
WASM is a runtime and language that's still in the initial stages of active development. It's therefore likely that support is limited and missing features could be discovered as this proposal is developed.

### wasm-bindgen doesn't currently support Rust traits
The wasm-bindgen crate doesn't currently support Rust traits, though it is a feature [under development](https://github.com/rustwasm/wasm-bindgen/pull/2871). This is slight limitation as the simple polymorphic trait functions that would be ideal for interfaces must first be wrapped in other functions in order to be compiled to WASM.

## Building and testing
To build and test this repository, build the WASM packages for nodejs while in the `program` directory
```
wasm-pack build --target nodejs
```

The *very simple* test program can then be executed using ts-node in the `program/node_test` directory.
```
ts-node test.js
```