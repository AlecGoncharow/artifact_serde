# artifact_serde [![Build Status](https://travis-ci.com/AlecGoncharow/artifact_serde.svg?branch=master)](https://travis-ci.com/AlecGoncharow/artifact_serde) [![crates.io](https://img.shields.io/crates/v/artifact_serde.svg)](https://crates.io/crates/artifact_serde) [![docs](https://docs.rs/artifact_serde/badge.svg)](https://docs.rs/artifact_serde/)

A small Rust library for serializing/deserialzing [Artifact](https://playartifact.com) Decks and Deck Codes. Created to support
my other Rust Artifact [library](https://github.com/AlecGoncharow/artifact_lib), but still provides its own use case for others.

# Usage
To install, add to `Cargo.toml`
```toml
[dependencies]
artifact_serde = "0.2.1"
```

If you are still on rust version < 1.31.0 you will need to add this to your crate root
```rust
extern crate artifact_serde
```

Basic usage example  
```rust
fn main() {
  let adc = "ADCJWkTZX05uwGDCRV4XQGy3QGLmqUBg4GQJgGLGgO7AaABR3JlZW4vQmxhY2sgRXhhbXBsZQ__";
  let mut my_deck = artifact_serde::decode(&adc).unwrap();
  
  // Should return a new String that is the same as the initial ADC
  let my_serialized_adc = artifact_serde::encode(&mut my_deck).unwrap();
}
```
`my_deck` will be an instance of `DeserializedDeck`, by itself it is not too useful, as you will still need to map the 
card ids to their respective cards. This is handled in my other [library](https://github.com/AlecGoncharow/artifact_lib), but if you wish to use this on its own follow
the instructions provided by Valve which is described in detail [here](https://github.com/ValveSoftware/ArtifactDeckCode#card-set-api)
